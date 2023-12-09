use anyhow::Context;

use std::convert::TryFrom;

use crate::core::Direction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node<'a> {
    pub label: &'a str,
    pub left: &'a str,
    pub right: &'a str,
}

impl<'a> Node<'a> {
    pub fn traverse(&'a self, direction: &Direction) -> &'a str {
        match direction {
            Direction::Left => self.left,
            Direction::Right => self.right,
        }
    }
}

impl<'a> TryFrom<&'a str> for Node<'a> {
    type Error = anyhow::Error;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let (lhs, rhs) = input
            .split_once('=')
            .context("expected node label/branches to be delimited with an equal sign")?;

        let label = lhs.trim();

        let (left, right) = rhs
            .split_once(',')
            .context("expected node left/right branches to be delimited with a comma")?;

        let left = left
            .trim()
            .strip_prefix('(')
            .context("missing opening parentheses")?;

        let right = right
            .trim()
            .strip_suffix(')')
            .context("missing closing parentheses")?;

        Ok(Self { label, left, right })
    }
}
