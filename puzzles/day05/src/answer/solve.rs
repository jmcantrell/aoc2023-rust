use anyhow::Context;

use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(almanac: &Parsed1) -> anyhow::Result<Solution1> {
    almanac
        .seeds
        .iter()
        .map(|&seed| {
            almanac
                .maps
                .iter()
                .fold(seed, |value, map| map.source_to_destination(value))
        })
        .min()
        .context("no solution")
}

pub fn solve2(almanac: &Parsed2) -> anyhow::Result<Solution2> {
    let seed_ranges: Vec<_> = almanac
        .seeds
        .chunks_exact(2)
        .map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .collect();

    (0..)
        .find(|&location| {
            seed_ranges.iter().any(|seed_range| {
                let seed = almanac
                    .maps
                    .iter()
                    .rev()
                    .fold(location, |value, map| map.destination_to_source(value));

                seed_range.contains(&seed)
            })
        })
        .context("no solution")
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn solve1() -> anyhow::Result<()> {
        assert_eq!(super::solve1(&parse1(INPUT)?)?, 35);
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        assert_eq!(super::solve2(&parse2(INPUT)?)?, 46);
        Ok(())
    }
}
