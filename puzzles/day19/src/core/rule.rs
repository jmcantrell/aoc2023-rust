use anyhow::Context;

use super::{Condition, Destination, Part};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule<'a> {
    pub condition: Condition,
    pub if_true: Destination<'a>,
}

impl Rule<'_> {
    pub fn test(&self, part: &Part) -> Option<Destination> {
        self.condition.test(part).then_some(self.if_true.clone())
    }
}

impl<'a> std::convert::TryFrom<&'a str> for Rule<'a> {
    type Error = anyhow::Error;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let (left, right) = input
            .split_once(':')
            .context("expected condition/result to be delimited by a colon")?;

        let condition = left.try_into().context("condition")?;

        let if_true = right.try_into()?;

        Ok(Self { condition, if_true })
    }
}
