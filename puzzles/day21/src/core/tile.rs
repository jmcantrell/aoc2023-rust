use anyhow::anyhow;

use Tile::*;

const GARDEN: char = '.';
const ROCK: char = '#';

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Garden,
    Rock,
}

impl Tile {
    pub fn is_garden(&self) -> bool {
        matches!(self, Garden)
    }

    pub fn to_char(&self) -> char {
        match self {
            Garden => GARDEN,
            Rock => ROCK,
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl std::convert::TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(input: char) -> Result<Self, Self::Error> {
        match input {
            GARDEN => Ok(Garden),
            ROCK => Ok(Rock),
            _ => Err(anyhow!("invalid tile: {:?}", input)),
        }
    }
}
