use anyhow::{ensure, Context};

use crate::core::Graph;

use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

#[cfg(test)]
const STEPS1: usize = 6;

#[cfg(test)]
const STEPS2: usize = 6;

#[cfg(not(test))]
const STEPS1: usize = 64;

#[cfg(not(test))]
const STEPS2: usize = 26501365;

// The first part is a brute-force flood filling that keeps track of the step parity as it grows.
// This will be used in part 2 to get solution values at various points for interpolation.

pub fn solve1(map: &Parsed1) -> anyhow::Result<Solution1> {
    map.iter_walk_reached_counts()
        .nth(STEPS1)
        .context("no solution")
}

// The input grid looks like the following, with the rocks naturally clustered into sections:
//
//     +---+---+    It has the following qualities:
//     |E /|\ F|
//     |/ A|B \|    1. The grid is a square.
//     +---S---+    2. The start is at the center.
//     |\ D|C /|    3. The start row/column is completely clear of rocks.
//     |H \|/ G|    4. There is a diamond shape area clear of rocks
//     +---+---+
//
// The diamond in the center (ABCD) will be referred to as D₀.
// Since this grid is tiled infinitely in all directions, if you visualize the adjacent grids,
// you'll notice that there is only one other distinct diamond (GHEF) which I'll call D₁.
//
//     +---+---+---+---+---+---+         +---+---+---+---+---+---+
//     |E /|\ F|E /|\ F|E /|\ F|         |E /|\ F|E / \ F|E /|\ F|
//     |/ A|B \|/ A|B \|/ A|B \|         |/ A|B \|/     \|/ A|B \|
//     +---S---+---S---+---S---+         +---S---+   D₀  +---S---+
//     |\ D|C /|\ D|C /|\ D|C /|         |\ D|C / \     / \ D|C /|
//     |H \|/ G|H \|/ G|H \|/ G|         |H \|/     \ /     \|/ G|
//     +---+---+---+---+---+---+         +---+   D₁  +   D₁  +---+
//     |E /|\ F|E / \ F|E /|\ F|         |E / \     / \     / \ F|
//     |/ A|B \|/     \|/ A|B \|         |/     \ /     \ /     \|
//     +---S---+   D₀  +---S---+    =    +   D₀  +   D₀  +   D₀  +
//     |\ D|C /|\     /|\ D|C /|         |\     / \     / \     /|
//     |H \|/ G|H \ / G|H \|/ G|         |H \ /     \ /     \ / G|
//     +---+---+---+---+---+---+         +---+   D₁  +   D₁  +---+
//     |E /|\ F|E /|\ F|E /|\ F|         |E /|\     / \     /|\ F|
//     |/ A|B \|/ A|B \|/ A|B \|         |/ A|B \ /     \ / A|B \|
//     +---S---+---S---+---S---+         +---S---+   D₀  +---S---+
//     |\ D|C /|\ D|C /|\ D|C /|         |\ D|C /|\     /|\ D|C /|
//     |H \|/ G|H \|/ G|H \|/ G|         |H \|/ G|H \ / G|H \|/ G|
//     +---+---+---+---+---+---+         +---+---+---+---+---+---+
//
// From visualizing the frontier growth when flood-filling steps:
//
//     1. The frontier grows in the rough shape of a diamond (property of flood-filling).
//     2. After `size/2` steps, the frontier has exactly covered the initial diamond D₀ (presumably
//        because of qualities 2 and 3 of the grid.
//     3. After another `size` steps, the frontier has exactly added the 8 adjacent diamonds.
//     4. Each `size` steps thereafter will add another "layer" of adjacent diamonds that are an
//        even split of the two types of diamond.
//
// Since the diamond is growing equally in all directions and is essentially a big square, its
// area is going to be the side length squared. This is quadratic growth.
//
//     f(x) = ax² + bx + c
//
// Given what I've recently learned about interpolation and from what I've seen of the flood-fill
// visualization, I'm not sure it's possible to find a formula to determine the solution at every
// possible step, but given that we're trying to find the solution at 26,501,365 steps, and that
// number can also be represented as 65 + 131 * 202300, this is convenient given what has been
// observed so far:
//
//     1. The width of the grid is 131.
//     2. The start is at the center (row 65, column 65).
//     3. After 65 steps, the initial D₀ is exactly covered.
//     4. After 131 more steps, the next layer of adjacent diamonds is covered.
//     5. With every additional 131 steps, another layer of adjacent diamonds is added.
//
// This suggests that the necessary even spacing of the x axis will be defined like:
//
//     65 + i * 131
//
// Which gives the first three step numbers to sample:
//
//     step₀ = 65
//     step₁ = 65 + 131
//     step₂ = 65 + 131 * 2
//
// And the step number we need a solution for:
//
//     stepₙ = 65 + 131 * 202300 = 26501365
//
// With n = 202300, once we have determined the interpolated f(x), f(n) should give the solution.

pub fn solve2(map: &Parsed2) -> anyhow::Result<Solution2> {
    // Ensure grid quality 1.
    let (height, width) = map.grid.shape();
    ensure!(height == width, "grid is not a square");

    let half = width / 2;

    // Ensure grid quality 2.
    let center = (half as isize, half as isize);
    ensure!(map.start == center, "start is not at the center");

    // Ensure grid quality 3 (for start row).
    ensure!(
        map.grid.row(half).iter().all(|tile| tile.is_garden()),
        "start row is not clear"
    );

    // Ensure grid quality 3 (for start column).
    ensure!(
        map.grid.column(half).iter().all(|tile| tile.is_garden()),
        "start column is not clear"
    );

    let mut walk = map.iter_walk_reached_counts().enumerate();

    let mut iter_ys = (0..).map(|i| {
        let steps = half + i * width;
        walk.find_map(|(i, reached_count)| (i == steps).then_some(reached_count))
            .unwrap() as isize
    });

    // Get 3 values corresponding to f(0), f(1), and f(2).
    let y0 = iter_ys.next().unwrap();
    let y1 = iter_ys.next().unwrap();
    let y2 = iter_ys.next().unwrap();

    let w = width as isize;
    let n = (STEPS2 as isize - w / 2) / w;

    // Given that f(x) = ax² + bx + c, we can solve the equations:
    //
    //     y₀ = ax₀² + bx₀ + c
    //     y₁ = ax₁² + bx₁ + c
    //     y₂ = ax₂² + bx₂ + c
    //
    // Substituting x₀ = 0, x₁ = 1, and x₂ = 2, these can be simplified to:
    //
    //     y₀ = c
    //     y₁ = a + b + c
    //     y₂ = 4a + 2b + c

    let c = y0;

    // Substituting y₀ in for c, we only need to solve the following equations:
    //
    //     y₁ =  a +  b + y₀
    //     y₂ = 4a + 2b + y₀
    //
    // One way to solve for a is to scale the first equation so that the b's cancel:
    //
    //     -2y₁ = -2a - 2b - 2y₀
    //       y₂ =  4a + 2b +  y₀
    //
    // Then adding them together, rearranging and solving for a gets:
    //
    //     y₂ - 2y₁ = 2a - y₀
    //     a = (y₀ -2y₁ + y₂) / 2

    let a = (y0 - 2 * y1 + y2) / 2;

    // In a similar fashion, we can scale the same two equations so that the a's cancel:
    //
    //     -4y₁ = -4a - 4b - 4y₀
    //       y₂ =  4a + 2b +  y₀
    //
    // Then adding them together, rearranging and solving for b gets:
    //
    //     y₂ - 4y₁ = -2b - 3y₀
    //     b = (-3y₀ + 4y₁ - y₂) / 2

    let b = (-3 * y0 + 4 * y1 - y2) / 2;

    // Reassembling the quadratic form with the desired value substituted for x:

    let y = a * n.pow(2) + b * n + c;

    Ok(y as usize)
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::parse1;

    use super::*;

    const INPUT: Input = include_str!("../../input-test");

    #[test]
    fn test_solve1() -> anyhow::Result<()> {
        assert_eq!(solve1(&parse1(INPUT)?)?, 16);
        Ok(())
    }
}
