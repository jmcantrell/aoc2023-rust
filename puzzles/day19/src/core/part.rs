use anyhow::Context;

use std::collections::HashMap;

use super::{Category, Rating};

use Category::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Part {
    pub extremely_cool_looking: Rating,
    pub musical: Rating,
    pub aerodynamic: Rating,
    pub shiny: Rating,
}

impl Part {
    pub fn get(&self, category: Category) -> Rating {
        match category {
            ExtremelyCoolLooking => self.extremely_cool_looking,
            Musical => self.musical,
            Aerodynamic => self.aerodynamic,
            Shiny => self.shiny,
        }
    }

    pub fn total_rating(&self) -> Rating {
        self.extremely_cool_looking + self.musical + self.aerodynamic + self.shiny
    }
}

type Map = HashMap<Category, Rating>;

impl std::convert::TryFrom<Map> for Part {
    type Error = anyhow::Error;

    fn try_from(mut input: Map) -> Result<Self, Self::Error> {
        let mut take = |key: Category| {
            input
                .remove(&key)
                .with_context(|| format!("missing category: {:?}", key))
        };

        let extremely_cool_looking = take(ExtremelyCoolLooking)?;
        let musical = take(Musical)?;
        let aerodynamic = take(Aerodynamic)?;
        let shiny = take(Shiny)?;

        Ok(Self {
            extremely_cool_looking,
            musical,
            aerodynamic,
            shiny,
        })
    }
}

impl std::convert::TryFrom<&str> for Part {
    type Error = anyhow::Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        fn parse_key_value(input: &str) -> anyhow::Result<(Category, Rating)> {
            let (left, right) = input
                .split_once('=')
                .context("expected key/value to be delimited by an equal sign")?;

            let category = left.trim().try_into()?;
            let rating = right.trim().parse()?;

            Ok((category, rating))
        }

        input
            .strip_prefix('{')
            .context("expected an opening curly bracket")?
            .strip_suffix('}')
            .context("expected a closing curly bracket")?
            .split(',')
            .enumerate()
            .map(|(i, item)| {
                parse_key_value(item).with_context(|| format!("item number {}", i + 1))
            })
            .collect::<Result<Map, _>>()?
            .try_into()
    }
}
