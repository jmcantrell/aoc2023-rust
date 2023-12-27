use anyhow::{ensure, Context};

use std::collections::{HashSet, VecDeque};

use super::{Direction, Location, Offset, Tile};

type Inner = nalgebra::DMatrix<Tile>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Contraption(Inner);

fn step((row, column): Location, (vertical, horizontal): Offset) -> Option<Location> {
    row.checked_add_signed(vertical)
        .zip(column.checked_add_signed(horizontal))
}

impl Contraption {
    fn contains(&self, (row, column): Location) -> bool {
        row < self.0.nrows() && column < self.0.ncols()
    }

    fn neighbor(&self, location: Location, direction: Direction) -> Option<Location> {
        step(location, direction.offset())
            .and_then(|adjacent| self.contains(adjacent).then_some(adjacent))
    }

    pub fn size(&self) -> (usize, usize) {
        self.0.shape()
    }

    pub fn count_energized(&self, start: Location, enter_from: Direction) -> usize {
        let mut frontier = VecDeque::from([(start, enter_from)]);
        let mut energized: HashSet<(Location, Direction)> = Default::default();

        while let Some((location, enter_from)) = frontier.pop_front() {
            if energized.insert((location, enter_from)) {
                for exit_from in self.0[location].transit(enter_from) {
                    if let Some(adjacent) = self.neighbor(location, exit_from) {
                        frontier.push_back((adjacent, exit_from.opposite()));
                    }
                }
            }
        }

        energized
            .into_iter()
            .map(|(location, _)| location)
            .collect::<HashSet<_>>()
            .len()
    }
}

impl std::ops::Index<Location> for Contraption {
    type Output = Tile;

    fn index(&self, location: Location) -> &Self::Output {
        &self.0[location]
    }
}

impl std::convert::TryFrom<&str> for Contraption {
    type Error = anyhow::Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let lines: Vec<_> = input.lines().collect();

        let height = lines.len();
        ensure!(height > 0, "empty input");

        let width = lines[0].len();
        ensure!(width > 0, "empty first line");

        let values: Vec<Tile> = lines
            .into_iter()
            .enumerate()
            .flat_map(|(i, line)| {
                line.chars().enumerate().map(move |(j, c)| {
                    c.try_into()
                        .with_context(|| format!("row number {}, column number {}", i + 1, j + 1))
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        let inner = Inner::from_row_iterator(height, width, values);

        Ok(Self(inner))
    }
}

impl std::fmt::Display for Contraption {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for values in self.0.row_iter() {
            for value in values.iter() {
                write!(f, "{}", value)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
