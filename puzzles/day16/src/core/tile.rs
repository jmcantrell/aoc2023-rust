use anyhow::anyhow;

use super::Direction;

use Direction::*;
use Tile::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Empty,
    ForwardMirror,
    BackwardMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

impl Tile {
    pub fn transit(&self, enter_from: Direction) -> Vec<Direction> {
        match self {
            Empty => vec![enter_from.opposite()],
            ForwardMirror => vec![match enter_from {
                North => West,
                East => South,
                South => East,
                West => North,
            }],
            BackwardMirror => vec![match enter_from {
                North => East,
                East => North,
                South => West,
                West => South,
            }],
            VerticalSplitter => match enter_from {
                North | South => vec![enter_from.opposite()],
                East | West => vec![North, South],
            },
            HorizontalSplitter => match enter_from {
                North | South => vec![East, West],
                East | West => vec![enter_from.opposite()],
            },
        }
    }
}

impl std::convert::TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(input: char) -> Result<Self, Self::Error> {
        match input {
            '.' => Ok(Empty),
            '/' => Ok(ForwardMirror),
            '\\' => Ok(BackwardMirror),
            '|' => Ok(VerticalSplitter),
            '-' => Ok(HorizontalSplitter),
            _ => Err(anyhow!("invalid tile: {:?}", input)),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Empty => '.',
                ForwardMirror => '/',
                BackwardMirror => '\\',
                VerticalSplitter => '|',
                HorizontalSplitter => '-',
            }
        )
    }
}
