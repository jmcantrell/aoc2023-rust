use anyhow::anyhow;

use std::convert::TryFrom;

use super::{Direction, DIRECTIONS};

pub const NEIGHBORS_LEN: usize = 4;

type Bits = u16;
type Edges = [bool; NEIGHBORS_LEN];

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,
    TopLeft,
    TopRight,
    Vertical,
    Horizontal,
    BottomLeft,
    BottomRight,
}

impl Tile {
    pub fn bits(&self) -> Bits {
        match self {
            Self::Empty => 0,
            Self::TopLeft => Direction::South | Direction::East,
            Self::TopRight => Direction::South | Direction::West,
            Self::Vertical => Direction::North | Direction::South,
            Self::Horizontal => Direction::West | Direction::East,
            Self::BottomLeft => Direction::North | Direction::East,
            Self::BottomRight => Direction::North | Direction::West,
        }
    }

    pub fn has_edge(&self, direction: Direction) -> bool {
        self.bits() & direction.bits() != 0
    }

    pub fn edges(&self) -> impl Iterator<Item = Direction> + '_ {
        DIRECTIONS
            .into_iter()
            .filter(|&direction| self.has_edge(direction))
    }
}

impl TryFrom<Edges> for Tile {
    type Error = anyhow::Error;

    fn try_from(edges: Edges) -> Result<Self, Self::Error> {
        match edges {
            [false, true, true, false] => Ok(Self::TopLeft),
            [false, false, true, true] => Ok(Self::TopRight),
            [true, false, true, false] => Ok(Self::Vertical),
            [false, true, false, true] => Ok(Self::Horizontal),
            [true, true, false, false] => Ok(Self::BottomLeft),
            [true, false, false, true] => Ok(Self::BottomRight),
            _ => Err(anyhow!(
                "invalid tile edges (north, west, east, south): {:?}",
                edges
            )),
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Self::Empty),
            'F' => Ok(Self::TopLeft),
            '7' => Ok(Self::TopRight),
            '|' => Ok(Self::Vertical),
            '-' => Ok(Self::Horizontal),
            'L' => Ok(Self::BottomLeft),
            'J' => Ok(Self::BottomRight),
            _ => Err(anyhow!("invalid tile: {:?}", c)),
        }
    }
}
