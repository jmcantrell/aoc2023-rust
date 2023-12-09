use anyhow::Context;

use std::collections::HashMap;
use std::convert::TryFrom;

use super::{Direction, Node};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkMap<'a> {
    pub instructions: Vec<Direction>,
    pub nodes: HashMap<&'a str, Node<'a>>,
}

impl<'a> NetworkMap<'a> {
    pub fn new(instructions: Vec<Direction>, nodes: Vec<Node<'a>>) -> Self {
        Self {
            instructions,
            nodes: nodes.into_iter().map(|node| (node.label, node)).collect(),
        }
    }

    pub fn iter_instructions(&self) -> impl Iterator<Item = &Direction> {
        self.instructions.iter().cycle()
    }
}

impl<'a> TryFrom<&'a str> for NetworkMap<'a> {
    type Error = anyhow::Error;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let (top, bottom) = input
            .split_once("\n\n")
            .context("expected instructions/nodes to be separated by two newlines")?;

        let instructions = top
            .chars()
            .enumerate()
            .map(|(i, c)| {
                c.try_into()
                    .with_context(|| format!("instruction number {}", i + 1))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let nodes = bottom
            .lines()
            .enumerate()
            .map(|(i, s)| {
                s.try_into()
                    .with_context(|| format!("node number {}", i + 1))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self::new(instructions, nodes))
    }
}
