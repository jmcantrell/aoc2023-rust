use aoc::Input;

use crate::core::Image;

pub type Parsed = Image;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

pub fn parse(input: Input) -> anyhow::Result<Parsed> {
    Ok(input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(column, c)| (c == '#').then_some((row, column)))
        })
        .collect())
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

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn test_parse() -> anyhow::Result<()> {
        dbg!(parse(INPUT)?);
        Ok(())
    }
}
