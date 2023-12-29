use crate::core::HeatLoss;

use super::{Parsed1, Parsed2};

type Solution = HeatLoss;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(map: &Parsed1) -> anyhow::Result<Solution1> {
    Ok(map.minimal_heat_loss(1, 3))
}

pub fn solve2(map: &Parsed2) -> anyhow::Result<Solution2> {
    Ok(map.minimal_heat_loss(4, 10))
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    use super::*;

    const INPUT1: Input = include_str!("../../input-test1.txt");
    const INPUT2: Input = include_str!("../../input-test2.txt");

    #[test]
    fn test_solve1() -> anyhow::Result<()> {
        assert_eq!(solve1(&parse1(INPUT1)?)?, 102);
        Ok(())
    }

    #[test]
    fn test_solve2() -> anyhow::Result<()> {
        assert_eq!(solve2(&parse2(INPUT1)?)?, 94);
        assert_eq!(solve2(&parse2(INPUT2)?)?, 71);
        Ok(())
    }
}
