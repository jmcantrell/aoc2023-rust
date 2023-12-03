use anyhow::Context;

use std::collections::HashMap;

use aoc::Input;

use crate::core::{Game, RGB};

type Parsed = Vec<Game>;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse(input: Input) -> anyhow::Result<Parsed> {
    fn parse_cubes(s: &str) -> anyhow::Result<(&str, usize)> {
        let (left, color) = s
            .trim()
            .split_once(' ')
            .context("expected cube count/color to be separated by a space")?;

        let count = left.parse()?;

        Ok((color, count))
    }

    fn parse_rgb(s: &str) -> anyhow::Result<RGB> {
        let mut map = s
            .split(',')
            .enumerate()
            .map(|(i, s)| parse_cubes(s).with_context(|| format!("color number {}", i + 1)))
            .collect::<Result<HashMap<_, _>, _>>()?;

        let red = map.remove("red").unwrap_or_default();
        let green = map.remove("green").unwrap_or_default();
        let blue = map.remove("blue").unwrap_or_default();

        Ok([red, green, blue])
    }

    fn parse_game(s: &str) -> anyhow::Result<Game> {
        let (left, right) = s
            .split_once(": ")
            .context("expected game id/subsets to be separated by a colon")?;

        let left = left
            .strip_prefix("Game ")
            .context("expected game prefix/id to be separated by a space")?;

        let id = left.parse()?;

        let subsets = right
            .split(';')
            .enumerate()
            .map(|(i, s)| parse_rgb(s).with_context(|| format!("subset number {}", i + 1)))
            .collect::<Result<Vec<_>, _>>()?;

        Ok((id, subsets))
    }

    input
        .lines()
        .enumerate()
        .map(|(i, s)| parse_game(s).with_context(|| format!("game number {}", i + 1)))
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

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn parse() -> anyhow::Result<()> {
        dbg!(super::parse(INPUT)?);
        Ok(())
    }
}
