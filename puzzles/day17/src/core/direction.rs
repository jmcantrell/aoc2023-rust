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
    pub fn offset(&self) -> Offset {
        match self {
            North => (-1, 0),
            East => (0, 1),
            South => (1, 0),
            West => (0, -1),
        }
    }

    pub fn turns(&self) -> [Direction; 3] {
        match self {
            North => [West, North, East],
            East => [North, East, South],
            South => [East, South, West],
            West => [South, West, North],
        }
    }
}
