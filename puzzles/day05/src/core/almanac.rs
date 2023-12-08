use anyhow::Context;

use std::convert::TryFrom;

use super::Map;

#[derive(Debug, Clone)]
pub struct Almanac {
    pub seeds: Vec<usize>,
    pub maps: Vec<Map>,
}

impl TryFrom<&str> for Almanac {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> anyhow::Result<Self> {
        let mut chunks = s.split("\n\n");

        let seeds = chunks
            .next()
            .context("missing seeds")?
            .split_once(':')
            .context("expected seeds name/values to be delimited by a colon")?
            .1
            .split_whitespace()
            .enumerate()
            .map(|(i, s)| s.parse().with_context(|| format!("seed number {}", i + 1)))
            .collect::<Result<Vec<_>, _>>()?;

        let maps = chunks
            .enumerate()
            .map(|(i, s)| {
                s.try_into()
                    .with_context(|| format!("map number {}", i + 1))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { seeds, maps })
    }
}
