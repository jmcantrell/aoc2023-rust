use anyhow::anyhow;

use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Spring {
    Operational,
    Broken,
    Unknown,
}

impl TryFrom<char> for Spring {
    type Error = anyhow::Error;

    fn try_from(input: char) -> Result<Self, Self::Error> {
        match input {
            '.' => Ok(Self::Operational),
            '#' => Ok(Self::Broken),
            '?' => Ok(Self::Unknown),
            _ => Err(anyhow!("invalid status: {:?}", input)),
        }
    }
}
