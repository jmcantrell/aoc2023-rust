use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

fn solve(plays: &Parsed1) -> anyhow::Result<Solution> {
    let mut plays = plays.clone();
    plays.sort_by(|a, b| a.0.cmp(&b.0).reverse());

    Ok(plays
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid))| bid * (i + 1))
        .sum())
}

pub fn solve1(plays: &Parsed1) -> anyhow::Result<Solution1> {
    solve(plays)
}

pub fn solve2(plays: &Parsed2) -> anyhow::Result<Solution2> {
    solve(plays)
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    use super::*;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn test_solve1() -> anyhow::Result<()> {
        assert_eq!(solve1(&parse1(INPUT)?)?, 6440);
        Ok(())
    }

    #[test]
    fn test_solve2() -> anyhow::Result<()> {
        assert_eq!(solve2(&parse2(INPUT)?)?, 5905);
        Ok(())
    }
}
