use anyhow::Context;

use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct Range {
    pub source_start: usize,
    pub destination_start: usize,
    pub length: usize,
}

fn a_to_b(value: usize, start_a: usize, start_b: usize, length: usize) -> Option<usize> {
    if start_a <= value && value < start_a + length {
        Some(value - start_a + start_b)
    } else {
        None
    }
}

impl Range {
    pub fn source_to_destination(&self, source: usize) -> Option<usize> {
        a_to_b(
            source,
            self.source_start,
            self.destination_start,
            self.length,
        )
    }

    pub fn destination_to_source(&self, destination: usize) -> Option<usize> {
        a_to_b(
            destination,
            self.destination_start,
            self.source_start,
            self.length,
        )
    }
}

impl TryFrom<&str> for Range {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut tokens = s.split_whitespace();

        let destination_start: usize = tokens
            .next()
            .context("missing destination start")?
            .parse()
            .context("destination start")?;

        let source_start: usize = tokens
            .next()
            .context("missing source start")?
            .parse()
            .context("source start")?;

        let length: usize = tokens
            .next()
            .context("missing length")?
            .parse()
            .context("length")?;

        Ok(Self {
            destination_start,
            source_start,
            length,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_to_destination() {
        let range = Range {
            destination_start: 50,
            source_start: 98,
            length: 2,
        };

        assert_eq!(range.source_to_destination(97), None);
        assert_eq!(range.source_to_destination(98), Some(50));
        assert_eq!(range.source_to_destination(99), Some(51));
        assert_eq!(range.source_to_destination(100), None);
    }
}
