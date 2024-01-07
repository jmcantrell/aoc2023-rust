use anyhow::Context;

use super::{Destination, Rule};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workflow<'a> {
    pub rules: Vec<Rule<'a>>,
    pub fallback: Destination<'a>,
}

impl<'a> std::convert::TryFrom<&'a str> for Workflow<'a> {
    type Error = anyhow::Error;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let mut tokens: Vec<_> = input.split(',').collect();

        let fallback = tokens.pop().context("missing fallback")?.try_into()?;

        let rules = tokens
            .into_iter()
            .enumerate()
            .map(|(i, token)| {
                token
                    .try_into()
                    .with_context(|| format!("rule number {}", i + 1))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { rules, fallback })
    }
}
