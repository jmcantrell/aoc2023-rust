use aoc::Input;

use crate::core::Field;

type Parsed = Field;
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

    const INPUT1A: Input = include_str!("../../input-test1a");
    const INPUT1B: Input = include_str!("../../input-test1b");
    const INPUT2A: Input = include_str!("../../input-test2a");
    const INPUT2B: Input = include_str!("../../input-test2b");
    const INPUT3: Input = include_str!("../../input-test3");
    const INPUT4: Input = include_str!("../../input-test4");
    const INPUT5: Input = include_str!("../../input-test5");
    const INPUT6: Input = include_str!("../../input-test6");

    #[test]
    fn test_parse() -> anyhow::Result<()> {
        dbg!(parse(INPUT1A)?);
        dbg!(parse(INPUT1B)?);
        dbg!(parse(INPUT2A)?);
        dbg!(parse(INPUT2B)?);
        dbg!(parse(INPUT3)?);
        dbg!(parse(INPUT4)?);
        dbg!(parse(INPUT5)?);
        dbg!(parse(INPUT6)?);
        Ok(())
    }
}
