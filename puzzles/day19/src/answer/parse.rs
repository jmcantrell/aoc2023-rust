use anyhow::Context;

use std::collections::HashMap;

use aoc::Input;

use crate::core::{Part, Workflow};

pub type Workflows = HashMap<&'static str, Workflow<'static>>;
pub type Parts = Vec<Part>;

type Parsed = (Workflows, Parts);
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse(input: Input) -> anyhow::Result<Parsed> {
    fn parse_key_value(input: &str) -> anyhow::Result<(&str, Workflow)> {
        let (label, right) = input
            .strip_suffix('}')
            .context("missing closing curly bracket")?
            .split_once('{')
            .context("missing opening curly bracket")?;

        let workflow = right.try_into().context("workflow")?;

        Ok((label, workflow))
    }

    let (top, bottom) = input
        .split_once("\n\n")
        .context("expected workflows/parts to be delimited by an empty line")?;

    let workflows = top
        .lines()
        .enumerate()
        .map(|(i, line)| {
            parse_key_value(line).with_context(|| format!("workflow number {}", i + 1))
        })
        .collect::<Result<HashMap<_, _>, _>>()?;

    let parts = bottom
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.try_into()
                .with_context(|| format!("part number {}", i + 1))
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok((workflows, parts))
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
