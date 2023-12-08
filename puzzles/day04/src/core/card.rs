use anyhow::Context;
use lazy_regex::{lazy_regex, Lazy, Regex};

use std::collections::HashSet;
use std::convert::TryFrom;

use super::{Identifier, Number, Score};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Card {
    pub identifier: Identifier,
    pub winning_numbers: HashSet<Number>,
    pub numbers_you_have: HashSet<Number>,
}

impl Card {
    pub fn matching_numbers(&self) -> impl Iterator<Item = Number> + '_ {
        self.winning_numbers
            .intersection(&self.numbers_you_have)
            .cloned()
    }

    pub fn score(&self) -> Score {
        let num_matches = self.matching_numbers().count();

        if num_matches > 0 {
            1 << (num_matches - 1)
        } else {
            0
        }
    }
}

impl TryFrom<&str> for Card {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        static RE: Lazy<Regex> = lazy_regex!(
            r"^Card\s+(?<identifier>\d+):(?<winning_numbers>.*)\|(?<numbers_you_have>.*)$"
        );

        let captures = RE
            .captures(s)
            .with_context(|| format!("expected input to match: {:?}", RE.as_str()))?;

        let identifier = captures["identifier"].parse()?;

        let winning_numbers = captures["winning_numbers"]
            .split_whitespace()
            .enumerate()
            .map(|(i, s)| {
                s.parse()
                    .with_context(|| format!("winning number {}", i + 1))
            })
            .collect::<Result<HashSet<_>, _>>()?;

        let numbers_you_have = captures["numbers_you_have"]
            .split_whitespace()
            .enumerate()
            .map(|(i, s)| {
                s.parse()
                    .with_context(|| format!("number you have {}", i + 1))
            })
            .collect::<Result<HashSet<_>, _>>()?;

        Ok(Self {
            identifier,
            winning_numbers,
            numbers_you_have,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    const INPUT: &str = include_str!("../../input-test.txt");

    #[test]
    fn matching_numbers() {
        macro_rules! test {
            ($expected:expr) => {
                assert_eq!(
                    INPUT
                        .lines()
                        .map(|s| super::Card::try_from(s)
                            .unwrap()
                            .matching_numbers()
                            .collect::<HashSet<_>>())
                        .collect::<Vec<_>>(),
                    $expected
                        .into_iter()
                        .map(|numbers| numbers.into_iter().collect::<HashSet<_>>())
                        .collect::<Vec<_>>()
                );
            };
        }

        test!([
            vec![48, 83, 17, 86],
            vec![32, 61],
            vec![1, 21],
            vec![84],
            vec![],
            vec![],
        ]);
    }

    #[test]
    fn score() {
        macro_rules! test {
            ($expected:expr) => {
                assert_eq!(
                    INPUT
                        .lines()
                        .map(|s| super::Card::try_from(s).unwrap().score())
                        .collect::<Vec<_>>(),
                    $expected.into_iter().collect::<Vec<_>>()
                );
            };
        }

        test!([8, 2, 2, 1, 0, 0]);
    }
}
