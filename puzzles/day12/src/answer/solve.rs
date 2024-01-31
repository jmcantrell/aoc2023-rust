use std::collections::HashMap;

use crate::core::Spring;

use super::{Parsed1, Parsed2};

use Spring::*;

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

// Adapted from: https://github.com/michaeljgallagher/Advent-of-Code/blob/master/2023/12.py
fn count_possible_arrangements<'a>(springs: &'a [Spring], groups: &'a [usize]) -> usize {
    fn recurse<'a>(
        cache: &mut HashMap<(&'a [Spring], &'a [usize]), usize>,
        springs: &'a [Spring],
        groups: &'a [usize],
    ) -> usize {
        let key = (springs, groups);

        if let Some(value) = cache.get(&key) {
            return *value;
        }

        if springs.is_empty() {
            return if groups.is_empty() { 1 } else { 0 };
        }

        if groups.is_empty() {
            return if springs.contains(&Broken) { 0 } else { 1 };
        }

        let mut result = 0;

        let first_spring = springs[0];

        if matches!(first_spring, Operational | Unknown) {
            result += recurse(cache, &springs[1..], groups);
        }

        let first_group = groups[0];

        if matches!(first_spring, Broken | Unknown)
            && first_group <= springs.len()
            && !springs[..first_group].contains(&Operational)
            && (first_group == springs.len() || springs[first_group] != Broken)
        {
            result += recurse(
                cache,
                &springs[(first_group + 1).min(springs.len())..],
                &groups[1..],
            );
        }

        cache.insert(key, result);

        result
    }

    recurse(&mut HashMap::new(), springs, groups)
}

pub fn solve1(records: &Parsed1) -> anyhow::Result<Solution1> {
    Ok(records
        .iter()
        .map(|(springs, groups)| count_possible_arrangements(springs, groups))
        .sum())
}

pub fn solve2(records: &Parsed2) -> anyhow::Result<Solution2> {
    let folded_len = 5;

    Ok(records
        .iter()
        .map(|(springs, groups)| {
            let n = springs.len();
            let m = groups.len();

            let springs = springs
                .clone()
                .into_iter()
                .chain(std::iter::once(Unknown))
                .cycle()
                .take(n * folded_len + folded_len - 1)
                .collect::<Vec<_>>();

            let groups = groups
                .clone()
                .into_iter()
                .cycle()
                .take(m * folded_len)
                .collect::<Vec<_>>();

            count_possible_arrangements(&springs, &groups)
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
        assert_eq!(solve1(&parse1(INPUT)?)?, 21);
        Ok(())
    }

    #[test]
    fn test_solve2() -> anyhow::Result<()> {
        assert_eq!(solve2(&parse2(INPUT)?)?, 525_152);
        Ok(())
    }
}
