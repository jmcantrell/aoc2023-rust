use crate::core::{Instruction, Point};

use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

fn solve(instructions: &[Instruction]) -> usize {
    let vertices: Vec<_> = instructions
        .iter()
        .scan(Point::from([0, 0]), |prev_point, instruction| {
            let vector = instruction.direction.unit() * instruction.distance as isize;
            let next_point = *prev_point + vector;
            *prev_point = next_point;
            Some(next_point)
        })
        .collect();

    // https://en.wikipedia.org/wiki/Shoelace_formula
    let area = vertices
        .iter()
        .zip(vertices.iter().cycle().skip(1).take(vertices.len() + 1))
        .map(|(&a, &b)| (a.x * b.y) - (a.y * b.x))
        .sum::<isize>()
        .unsigned_abs()
        / 2;

    let perimeter: usize = instructions
        .iter()
        .map(|instruction| instruction.distance)
        .sum();

    // https://en.m.wikipedia.org/wiki/Pick%27s_theorem
    let inner = area - (perimeter / 2) + 1;

    inner + perimeter
}

pub fn solve1(instructions: &Parsed1) -> anyhow::Result<Solution1> {
    Ok(solve(instructions))
}

pub fn solve2(instructions: &Parsed2) -> anyhow::Result<Solution2> {
    Ok(solve(instructions))
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    use super::*;

    const INPUT: Input = include_str!("../../input-test");

    #[test]
    fn test_solve1() -> anyhow::Result<()> {
        assert_eq!(solve1(&parse1(INPUT)?)?, 62);
        Ok(())
    }

    #[test]
    fn test_solve2() -> anyhow::Result<()> {
        assert_eq!(solve2(&parse2(INPUT)?)?, 952_408_144_115);
        Ok(())
    }
}
