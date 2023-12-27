use anyhow::Context;

use super::Operation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Step<'a> {
    pub label: &'a str,
    pub operation: Operation,
}

impl<'a> std::convert::TryFrom<&'a str> for Step<'a> {
    type Error = anyhow::Error;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let i = input
            .find(|c: char| c == '=' || c == '-')
            .context("expected value to start with a '=' or '-'")?;

        let (label, rest) = input.split_at(i);

        let operation = rest.try_into().context("value")?;

        Ok(Self { label, operation })
    }
}
