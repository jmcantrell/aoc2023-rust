use anyhow::Context;

use std::convert::TryFrom;

use super::Range;

type Inner = Vec<Range>;

#[derive(Debug, Clone)]
pub struct Map(Inner);

impl Map {
    pub fn source_to_destination(&self, value: usize) -> usize {
        self.0
            .iter()
            .find_map(|range| range.source_to_destination(value))
            .unwrap_or(value)
    }

    pub fn destination_to_source(&self, value: usize) -> usize {
        self.0
            .iter()
            .find_map(|range| range.destination_to_source(value))
            .unwrap_or(value)
    }
}

impl From<Inner> for Map {
    fn from(ranges: Inner) -> Self {
        Self(ranges)
    }
}

impl TryFrom<&str> for Map {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(s.lines()
            .skip(1)
            .enumerate()
            .map(|(i, s)| {
                s.try_into()
                    .with_context(|| format!("range number {}", i + 1))
            })
            .collect::<Result<Vec<_>, _>>()?
            .into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translate() {
        let map = Map(vec![
            Range {
                destination_start: 50,
                source_start: 98,
                length: 2,
            },
            Range {
                destination_start: 52,
                source_start: 50,
                length: 48,
            },
        ]);

        assert_eq!(map.source_to_destination(79), 81);
        assert_eq!(map.source_to_destination(14), 14);
        assert_eq!(map.source_to_destination(55), 57);
        assert_eq!(map.source_to_destination(13), 13);
    }
}
