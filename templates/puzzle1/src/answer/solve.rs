use super::Parsed;

pub type Solution = usize;

pub fn solve(parsed: &Parsed) -> anyhow::Result<Solution> {
    todo!()
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::parse;

    use super::*;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn test_solve() -> anyhow::Result<()> {
        assert_eq!(solve(&parse(INPUT)?)?, 0);
        Ok(())
    }
}
