use anyhow::{ensure, Context};

use super::{Category, Operator, Part, Rating};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Condition {
    pub category: Category,
    pub operator: Operator,
    pub rating: Rating,
}

impl Condition {
    pub fn test(&self, part: &Part) -> bool {
        self.operator.compare(part.get(self.category), self.rating)
    }

    pub fn inverse(&self) -> Self {
        Self {
            category: self.category,
            operator: self.operator.inverse(),
            rating: self.rating,
        }
    }
}

impl std::convert::TryFrom<&str> for Condition {
    type Error = anyhow::Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        ensure!(
            input.len() >= 3,
            "expected input to be at least 3 characters"
        );

        let (prefix, input) = input.split_at(1);
        let category = prefix.try_into()?;

        let (prefix, input) = input.split_at(1);
        let operator = prefix.try_into()?;

        let rating = input.parse().context("rating")?;

        Ok(Self {
            category,
            operator,
            rating,
        })
    }
}
