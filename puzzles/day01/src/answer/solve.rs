use std::collections::HashMap;

use crate::core::Value;

use super::{Parsed1, Parsed2};

type Solution = Value;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

fn calibration_value(mut digits: impl Iterator<Item = Value>) -> Value {
    let first = digits.next().unwrap();
    let last = digits.last().unwrap_or(first);

    first * 10 + last
}

fn solve(lines_of_digits: impl Iterator<Item = impl Iterator<Item = Value>>) -> Solution {
    lines_of_digits.map(calibration_value).sum()
}

pub fn solve1(lines: &Parsed1) -> anyhow::Result<Solution1> {
    let lines_of_digits = lines
        .iter()
        .map(|s| s.chars().filter_map(|c| c.to_digit(10)));

    Ok(solve(lines_of_digits))
}

pub fn solve2(lines: &Parsed2) -> anyhow::Result<Solution2> {
    let valid_digits = [
        (1, "1", "one"),
        (2, "2", "two"),
        (3, "3", "three"),
        (4, "4", "four"),
        (5, "5", "five"),
        (6, "6", "six"),
        (7, "7", "seven"),
        (8, "8", "eight"),
        (9, "9", "nine"),
    ]
    .into_iter()
    .fold(HashMap::new(), |mut acc, (digit, s, name)| {
        acc.insert(s, digit);
        acc.insert(name, digit);
        acc
    });

    fn substrings(s: &str) -> impl Iterator<Item = &str> {
        (0..s.len()).flat_map(move |i| (i + 1..=s.len()).map(move |j| &s[i..j]))
    }

    let lines_of_digits = lines
        .iter()
        .map(|s| substrings(s).filter_map(|s| valid_digits.get(s).cloned()));

    Ok(solve(lines_of_digits))
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    use super::*;

    const INPUT1: Input = include_str!("../../input-test1");
    const INPUT2: Input = include_str!("../../input-test2");

    #[test]
    fn test_solve1() -> anyhow::Result<()> {
        assert_eq!(solve1(&parse1(INPUT1)?)?, 142);
        Ok(())
    }

    #[test]
    fn test_solve2() -> anyhow::Result<()> {
        assert_eq!(solve2(&parse2(INPUT2)?)?, 281);
        Ok(())
    }
}
