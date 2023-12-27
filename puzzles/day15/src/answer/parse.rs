use anyhow::Context;

use aoc::Input;

use crate::core::Step;

pub type Parsed1 = Vec<Input>;
pub type Parsed2 = Vec<Step<'static>>;

pub fn parse1(input: Input) -> anyhow::Result<Parsed1> {
    Ok(input.trim().split(',').collect())
}

pub fn parse2(input: Input) -> anyhow::Result<Parsed2> {
    input
        .trim()
        .split(',')
        .enumerate()
        .map(|(i, input)| {
            input
                .try_into()
                .with_context(|| format!("step number {}", i + 1))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use super::*;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn test_parse1() -> anyhow::Result<()> {
        dbg!(parse1(INPUT)?);
        Ok(())
    }

    #[test]
    fn test_parse2() -> anyhow::Result<()> {
        dbg!(parse2(INPUT)?);
        Ok(())
    }
}
