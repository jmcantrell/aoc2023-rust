use super::Offset;

use Direction::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }

    pub fn offset(&self) -> Offset {
        match self {
            North => (-1, 0),
            East => (0, 1),
            South => (1, 0),
            West => (0, -1),
        }
    }
}
