use aoc::Input;

use crate::core::Map;

type Parsed = Map;
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

    const INPUT1: Input = include_str!("../../input-test1.txt");
    const INPUT2: Input = include_str!("../../input-test2.txt");

    #[test]
    fn test_parse() -> anyhow::Result<()> {
        dbg!(parse(INPUT1)?);
        dbg!(parse(INPUT2)?);
        Ok(())
    }

    #[test]
    fn test_identity() {
        assert_eq!(format!("{}", parse(INPUT1).unwrap()), INPUT1);
        assert_eq!(format!("{}", parse(INPUT2).unwrap()), INPUT2);
    }
}
