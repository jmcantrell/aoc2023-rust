use anyhow::{anyhow, ensure};

use super::Rating;

use Operator::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    LessThan,
    GreaterThan,
}

impl Operator {
    pub fn compare(&self, a: Rating, b: Rating) -> bool {
        match self {
            LessThan => a < b,
            GreaterThan => a > b,
        }
    }

    pub fn inverse(&self) -> Self {
        match self {
            LessThan => GreaterThan,
            GreaterThan => LessThan,
        }
    }
}

impl std::convert::TryFrom<char> for Operator {
    type Error = anyhow::Error;

    fn try_from(input: char) -> Result<Self, Self::Error> {
        match input {
            '<' => Ok(LessThan),
            '>' => Ok(GreaterThan),
            _ => Err(anyhow!("invalid operator: {:?}", input)),
        }
    }
}

impl std::convert::TryFrom<&str> for Operator {
    type Error = anyhow::Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        ensure!(
            input.len() == 1,
            "expected input to be a single character, but it was {} characters",
            input.len()
        );

        input.chars().next().unwrap().try_into()
    }
}
