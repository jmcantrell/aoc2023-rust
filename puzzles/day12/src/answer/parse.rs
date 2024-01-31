use anyhow::Context;

use aoc::Input;

use crate::core::Record;

type Parsed = Vec<Record>;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse(input: Input) -> anyhow::Result<Parsed> {
    fn parse_record(input: &str) -> anyhow::Result<Record> {
        let (left, right) = input
            .split_once(|c: char| c.is_whitespace())
            .context("expected springs and counts to be delimited by white space")?;

        let springs = left
            .chars()
            .enumerate()
            .map(|(i, c)| {
                c.try_into()
                    .with_context(|| format!("spring number {}", i + 1))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let groups = right
            .split(',')
            .enumerate()
            .map(|(i, s)| {
                s.trim()
                    .parse()
                    .with_context(|| format!("group number {}", i + 1))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok((springs, groups))
    }

    input
        .lines()
        .enumerate()
        .map(|(i, line)| parse_record(line).with_context(|| format!("record number {}", i + 1)))
        .collect::<Result<Vec<_>, _>>()
}

pub fn parse1(input: Input) -> anyhow::Result<Parsed1> {
    parse(input)
}

pub fn parse2(input: Input) -> anyhow::Result<Parsed2> {
    parse(input)
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use super::*;

    const INPUT: Input = include_str!("../../input-test");

    #[test]
    fn test_parse() -> anyhow::Result<()> {
        dbg!(parse(INPUT)?);
        Ok(())
    }
}
