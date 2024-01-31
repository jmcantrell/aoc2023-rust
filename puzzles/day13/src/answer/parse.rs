use anyhow::Context;

use aoc::Input;

use crate::core::Grid;

type Parsed = Vec<Grid>;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

pub fn parse(input: Input) -> anyhow::Result<Parsed> {
    input
        .split("\n\n")
        .enumerate()
        .map(|(i, chunk)| {
            chunk
                .try_into()
                .with_context(|| format!("grid number {}", i + 1))
        })
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
