use super::Parsed;

pub type Solution = usize;

pub fn solve(parsed: &Parsed) -> anyhow::Result<Solution> {
    todo!()
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::parse;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn solve() -> anyhow::Result<()> {
        assert_eq!(super::solve(&parse(INPUT)?)?, 0);
        Ok(())
    }
}
