use anyhow::Context;

use crate::core::Grid;

use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

fn solve(grids: &[Grid], allowed_smudges: usize) -> anyhow::Result<Solution> {
    let mut result = 0;

    for (i, grid) in grids.iter().enumerate() {
        let possibilities = [
            (
                grid.reflection_diff_counts(grid.width(), grid.height(), |i, n| ((i, 0), (1, n))),
                1,
            ),
            (
                grid.reflection_diff_counts(grid.height(), grid.width(), |i, n| ((0, i), (n, 1))),
                100,
            ),
        ];

        let summary = possibilities
            .into_iter()
            .find_map(|(reflection_diff_counts, multiplier)| {
                reflection_diff_counts
                    .into_iter()
                    .find_map(|(index, diff_count)| {
                        (diff_count == allowed_smudges).then_some(multiplier * (index + 1))
                    })
            })
            .with_context(|| format!("grid number {} had no reflection line", i + 1))?;

        result += summary;
    }

    Ok(result)
}

pub fn solve1(grids: &Parsed1) -> anyhow::Result<Solution1> {
    solve(grids, 0)
}

pub fn solve2(grids: &Parsed2) -> anyhow::Result<Solution2> {
    solve(grids, 1)
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    use super::*;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn test_solve1() -> anyhow::Result<()> {
        assert_eq!(solve1(&parse1(INPUT)?)?, 405);
        Ok(())
    }

    #[test]
    fn test_solve2() -> anyhow::Result<()> {
        assert_eq!(solve2(&parse2(INPUT)?)?, 400);
        Ok(())
    }
}
