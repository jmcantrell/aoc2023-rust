use std::collections::{HashSet, VecDeque};

use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(snapshot: &Parsed1) -> anyhow::Result<Solution1> {
    let mut stack = snapshot.clone();
    stack.settle();

    let (supports, is_supported_by) = stack.dependencies();

    Ok(stack
        .bricks
        .iter()
        .filter(|brick| {
            is_supported_by[brick].iter().all(|brick_above| {
                supports[brick_above]
                    .iter()
                    .any(|brick_below| brick_below != brick)
            })
        })
        .count())
}

pub fn solve2(snapshot: &Parsed2) -> anyhow::Result<Solution2> {
    let mut stack = snapshot.clone();
    stack.settle();

    let (supports, is_supported_by) = stack.dependencies();

    Ok(stack
        .bricks
        .iter()
        .map(|start| {
            let mut frontier = VecDeque::from([start]);
            let mut dropping = HashSet::new();

            while let Some(brick) = frontier.pop_front() {
                dropping.insert(brick);
                for brick_above in is_supported_by[brick].iter() {
                    if supports[brick_above]
                        .iter()
                        .all(|brick_below| dropping.contains(brick_below))
                    {
                        frontier.push_back(brick_above);
                    }
                }
            }

            dropping.len() - 1
        })
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
        assert_eq!(solve1(&parse1(INPUT)?)?, 5);
        Ok(())
    }

    #[test]
    fn test_solve2() -> anyhow::Result<()> {
        assert_eq!(solve2(&parse2(INPUT)?)?, 7);
        Ok(())
    }
}
