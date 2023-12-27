use aoc::Input;

use crate::core::Contraption;

type Parsed = Contraption;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse(input: Input) -> anyhow::Result<Parsed> {
    input.try_into()
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
        dbg!("{}", parse(INPUT)?);
        Ok(())
    }

    #[test]
    fn test_identity() {
        assert_eq!(format!("{}", parse(INPUT).unwrap()), INPUT);
    }
}
