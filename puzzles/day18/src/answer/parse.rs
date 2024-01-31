use anyhow::{ensure, Context};

use aoc::Input;

use crate::core::Instruction;

type Parsed = Vec<Instruction>;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse_code(input: &str) -> anyhow::Result<(&str, &str, &str)> {
    let mut tokens = input.split_whitespace();

    let token1 = tokens.next().context("missing token 1")?;
    let token2 = tokens.next().context("missing token 2")?;
    let token3 = tokens.next().context("missing token 3")?;

    Ok((token1, token2, token3))
}

pub fn parse1(input: Input) -> anyhow::Result<Parsed1> {
    fn parse_instruction(input: &str) -> anyhow::Result<Instruction> {
        let (first, second, _) = parse_code(input)?;

        let direction = first.try_into().context("direction")?;
        let distance = second.parse().context("distance")?;

        Ok(Instruction {
            direction,
            distance,
        })
    }

    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            parse_instruction(line).with_context(|| format!("instruction number {}", i + 1))
        })
        .collect()
}

pub fn parse2(input: Input) -> anyhow::Result<Parsed2> {
    fn parse_instruction(input: &str) -> anyhow::Result<Instruction> {
        let (_, _, encoded) = parse_code(input)?;

        let encoded = encoded
            .strip_prefix('(')
            .context("missing opening parentheses")?
            .strip_suffix(')')
            .context("missing closing parentheses")?
            .strip_prefix('#')
            .context("missing hash")?;

        ensure!(
            encoded.len() == 6,
            "expected hexadecimal code to be 6 digits, but it was {}",
            encoded.len()
        );

        let (first, second) = encoded.split_at(5);

        let distance = usize::from_str_radix(first, 16)?;
        let direction = second.parse::<u8>()?.try_into()?;

        Ok(Instruction {
            direction,
            distance,
        })
    }

    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            parse_instruction(line).with_context(|| format!("instruction number {}", i + 1))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use super::*;

    const INPUT: Input = include_str!("../../input-test");

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
