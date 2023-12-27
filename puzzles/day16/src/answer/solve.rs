use anyhow::Context;

use crate::core::Direction;

use super::{Parsed1, Parsed2};

use Direction::*;

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(contraption: &Parsed1) -> anyhow::Result<Solution1> {
    Ok(contraption.count_energized((0, 0), West))
}

pub fn solve2(contraption: &Parsed2) -> anyhow::Result<Solution2> {
    contraption
        .edge_locations()
        .into_iter()
        .flat_map(|(enter_from, locations)| {
            locations
                .into_iter()
                .map(move |start| contraption.count_energized(start, enter_from))
        })
        .max()
        .context("no solution")
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    use super::*;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn test_solve1() -> anyhow::Result<()> {
        assert_eq!(solve1(&parse1(INPUT)?)?, 46);
        Ok(())
    }

    #[test]
    fn test_solve2() -> anyhow::Result<()> {
        assert_eq!(solve2(&parse2(INPUT)?)?, 51);
        Ok(())
    }
}
