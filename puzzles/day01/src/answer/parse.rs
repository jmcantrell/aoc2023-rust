use aoc::Input;

type Parsed = Vec<Input>;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

pub fn parse(input: Input) -> anyhow::Result<Parsed> {
    Ok(input.lines().collect())
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

    const INPUT1: Input = include_str!("../../input-test1");
    const INPUT2: Input = include_str!("../../input-test2");

    #[test]
    fn test_parse1() -> anyhow::Result<()> {
        dbg!(parse(INPUT1)?);
        Ok(())
    }

    #[test]
    fn test_parse() -> anyhow::Result<()> {
        dbg!(parse(INPUT2)?);
        Ok(())
    }
}
