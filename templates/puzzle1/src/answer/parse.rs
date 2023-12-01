use aoc::Input;

pub type Parsed = usize;

pub fn parse(input: Input) -> anyhow::Result<Parsed> {
    todo!()
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
