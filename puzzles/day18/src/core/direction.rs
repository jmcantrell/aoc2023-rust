use anyhow::{anyhow, ensure};

use super::Vector;

use Direction::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn unit(&self) -> Vector {
        match self {
            Up => [0, 1],
            Down => [0, -1],
            Left => [-1, 0],
            Right => [1, 0],
        }
        .into()
    }
}

impl std::convert::TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(input: char) -> Result<Self, Self::Error> {
        match input {
            'U' => Ok(Up),
            'D' => Ok(Down),
            'L' => Ok(Left),
            'R' => Ok(Right),
            _ => Err(anyhow!("invalid direction: {:?}", input)),
        }
    }
}

impl std::convert::TryFrom<&str> for Direction {
    type Error = anyhow::Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        ensure!(input.len() == 1, "input should be a single character");
        input.chars().next().unwrap().try_into()
    }
}

impl std::convert::TryFrom<u8> for Direction {
    type Error = anyhow::Error;

    fn try_from(input: u8) -> Result<Self, Self::Error> {
        match input {
            0 => Ok(Right),
            1 => Ok(Down),
            2 => Ok(Left),
            3 => Ok(Up),
            _ => Err(anyhow!("invalid direction: {:?}", input)),
        }
    }
}
