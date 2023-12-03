use crate::core::RGB_LEN;

use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(games: &Parsed1) -> anyhow::Result<Solution1> {
    let available_cubes = [12, 13, 14];

    Ok(games
        .iter()
        .filter_map(|(id, subsets)| {
            subsets
                .iter()
                .all(|subset| {
                    subset
                        .iter()
                        .zip(available_cubes.iter())
                        .all(|(count, total)| count <= total)
                })
                .then_some(id)
        })
        .sum())
}

pub fn solve2(games: &Parsed2) -> anyhow::Result<Solution2> {
    Ok(games
        .iter()
        .map(|(_, subsets)| {
            (0..RGB_LEN)
                .map(|i| {
                    subsets
                        .iter()
                        .map(|subset| subset[i])
                        .max()
                        .unwrap_or_default()
                })
                .product::<usize>()
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn solve1() -> anyhow::Result<()> {
        assert_eq!(super::solve1(&parse1(INPUT)?)?, 8);
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        assert_eq!(super::solve2(&parse2(INPUT)?)?, 2286);
        Ok(())
    }
}
