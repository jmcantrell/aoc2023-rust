use crate::core::Value;

use super::{Parsed1, Parsed2};

type Solution = Value;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(histories: &Parsed1) -> anyhow::Result<Solution1> {
    Ok(histories
        .iter()
        .map(|history| history.extrapolate_next().next().unwrap())
        .sum())
}

pub fn solve2(histories: &Parsed2) -> anyhow::Result<Solution2> {
    Ok(histories
        .iter()
        .map(|history| history.extrapolate_prev().next().unwrap())
        .sum())
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    use super::*;

    const INPUT: Input = include_str!("../../input-test");

    #[test]
    fn test_solve1() -> anyhow::Result<()> {
        assert_eq!(solve1(&parse1(INPUT)?)?, 114);
        Ok(())
    }

    #[test]
    fn test_solve2() -> anyhow::Result<()> {
        assert_eq!(solve2(&parse2(INPUT)?)?, 2);
        Ok(())
    }
}
