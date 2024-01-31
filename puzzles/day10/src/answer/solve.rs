use crate::core::Tile;

use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(field: &Parsed1) -> anyhow::Result<Solution1> {
    // I have no mathematical information to prove it yet, but I think that any possible loop must
    // be an even length, especially since we can only move vertically and horizontally. This means
    // that the farthest you could get from any point on the loop is half the length of the loop.
    Ok(field.loop_path().len() / 2)
}

pub fn solve2(field: &Parsed2) -> anyhow::Result<Solution2> {
    let loop_locations: Vec<_> = field.loop_path();

    let loop_vertices: Vec<_> = loop_locations
        .iter()
        .filter(|&&location| !matches!(field.grid[location], Tile::Vertical | Tile::Horizontal))
        .collect();

    let shoelace_pairs = loop_vertices.iter().zip(
        loop_vertices
            .iter()
            .cycle()
            .skip(1)
            .take(loop_vertices.len() + 1),
    );

    // https://en.wikipedia.org/wiki/Shoelace_formula
    let area = shoelace_pairs
        .map(|(&&a, &&b)| (a.0 * b.1) as isize - (a.1 * b.0) as isize)
        .sum::<isize>()
        .unsigned_abs()
        / 2;

    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    Ok(area - (loop_locations.len() / 2) + 1)
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    use super::*;

    const INPUT1A: Input = include_str!("../../input-test1a");
    const INPUT1B: Input = include_str!("../../input-test1b");
    const INPUT2A: Input = include_str!("../../input-test2a");
    const INPUT2B: Input = include_str!("../../input-test2b");
    const INPUT3: Input = include_str!("../../input-test3");
    const INPUT4: Input = include_str!("../../input-test4");
    const INPUT5: Input = include_str!("../../input-test5");
    const INPUT6: Input = include_str!("../../input-test6");

    #[test]
    fn test_solve1() -> anyhow::Result<()> {
        assert_eq!(solve1(&parse1(INPUT1A)?)?, 4);
        assert_eq!(solve1(&parse1(INPUT1B)?)?, 4);
        assert_eq!(solve1(&parse1(INPUT2A)?)?, 8);
        assert_eq!(solve1(&parse1(INPUT2B)?)?, 8);
        Ok(())
    }

    #[test]
    fn test_solve2() -> anyhow::Result<()> {
        assert_eq!(solve2(&parse2(INPUT1A)?)?, 1);
        assert_eq!(solve2(&parse2(INPUT1B)?)?, 1);
        assert_eq!(solve2(&parse2(INPUT2A)?)?, 1);
        assert_eq!(solve2(&parse2(INPUT2B)?)?, 1);
        assert_eq!(solve2(&parse2(INPUT3)?)?, 4);
        assert_eq!(solve2(&parse2(INPUT4)?)?, 4);
        assert_eq!(solve2(&parse2(INPUT5)?)?, 8);
        assert_eq!(solve2(&parse2(INPUT6)?)?, 10);
        Ok(())
    }
}
