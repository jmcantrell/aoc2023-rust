use std::ops::{BitAnd, BitOr};

use super::Offset;

use Direction::*;

pub const DIRECTIONS: [Direction; 4] = [North, East, South, West];

type Bits = u16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub const fn offset(&self) -> Offset {
        match self {
            North => (-1, 0),
            East => (0, 1),
            South => (1, 0),
            West => (0, -1),
        }
    }

    pub const fn bits(&self) -> Bits {
        match self {
            North => 0b1000,
            East => 0b0010,
            South => 0b0001,
            West => 0b0100,
        }
    }

    pub const fn opposite(&self) -> Self {
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
}

impl BitOr for Direction {
    type Output = Bits;

    fn bitor(self, other: Self) -> Self::Output {
        self.bits() | other.bits()
    }
}

impl BitAnd for Direction {
    type Output = Bits;

    fn bitand(self, other: Self) -> Self::Output {
        self.bits() & other.bits()
    }
}
