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

    const INPUT1: Input = include_str!("../../input-test1.txt");
    const INPUT2: Input = include_str!("../../input-test2.txt");

    #[test]
    fn parse1() -> anyhow::Result<()> {
        dbg!(super::parse(INPUT1)?);
        Ok(())
    }

    #[test]
    fn parse() -> anyhow::Result<()> {
        dbg!(super::parse(INPUT2)?);
        Ok(())
    }
}
