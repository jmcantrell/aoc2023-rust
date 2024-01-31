use aoc::Input;

pub type Parsed = usize;

pub fn parse(input: Input) -> anyhow::Result<Parsed> {
    todo!()
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
