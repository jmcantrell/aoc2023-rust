use crate::core::Value;

use super::{Parsed1, Parsed2};

type Solution = Value;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(schematic: &Parsed1) -> anyhow::Result<Solution1> {
    Ok(schematic.part_numbers().sum())
}

pub fn solve2(schematic: &Parsed2) -> anyhow::Result<Solution2> {
    Ok(schematic.gear_ratios().sum())
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    use super::*;

    const INPUT: Input = include_str!("../../input-test");

    #[test]
    fn test_solve1() -> anyhow::Result<()> {
        assert_eq!(solve1(&parse1(INPUT)?)?, 4361);
        Ok(())
    }

    #[test]
    fn test_solve2() -> anyhow::Result<()> {
        assert_eq!(solve2(&parse2(INPUT)?)?, 467835);
        Ok(())
    }
}
