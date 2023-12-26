use anyhow::{ensure, Context};
use nalgebra::DMatrix;

use super::{Direction, Location, Tile};

type Inner = DMatrix<Tile>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Platform(Inner);

fn step((row, column): Location, (vertical, horizontal): (isize, isize)) -> Option<Location> {
    row.checked_add_signed(vertical)
        .zip(column.checked_add_signed(horizontal))
}

impl Platform {
    pub fn tilt(&self, direction: &Direction) -> Self {
        let offset = direction.unit();
        let (height, width) = self.0.shape();

        let mut inner_new = self.0.clone();

        let roll = |mut location| {
            if inner_new[location] == Tile::RoundRock {
                while let Some(adjacent_location) = step(location, offset) {
                    if let Some(adjacent_tile) = inner_new.get(adjacent_location) {
                        if adjacent_tile == &Tile::Empty {
                            inner_new.swap(location, adjacent_location);
                            location = adjacent_location;
                            continue;
                        }
                    }
                    return;
                }
            }
        };

        match direction {
            Direction::North => (0..height)
                .flat_map(|row| (0..width).map(move |column| (row, column)))
                .for_each(roll),
            Direction::East => (0..width)
                .rev()
                .flat_map(|column| (0..height).map(move |row| (row, column)))
                .for_each(roll),
            Direction::South => (0..height)
                .rev()
                .flat_map(|row| (0..width).map(move |column| (row, column)))
                .for_each(roll),
            Direction::West => (0..width)
                .flat_map(|column| (0..height).map(move |row| (row, column)))
                .for_each(roll),
        };

        Self(inner_new)
    }

    pub fn total_load(&self) -> usize {
        let height = self.0.nrows();

        (0..height)
            .map(|row| {
                self.0
                    .row(row)
                    .iter()
                    .filter(|&&tile| tile == Tile::RoundRock)
                    .count()
                    * (height - row)
            })
            .sum()
    }
}

impl std::convert::TryFrom<&str> for Platform {
    type Error = anyhow::Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let lines: Vec<_> = input.lines().collect();

        let height = lines.len();
        ensure!(height > 0, "no rows");

        let width = lines.first().unwrap().len();
        ensure!(width > 0, "no columns in first row");

        for (i, line) in lines.iter().enumerate() {
            ensure!(
                line.len() == width,
                "expected line number {} to be {} characters, but it was {}",
                i + 1,
                width,
                line.len()
            );
        }

        let values = lines
            .into_iter()
            .enumerate()
            .flat_map(|(i, line)| {
                line.chars().enumerate().map(move |(j, c)| {
                    c.try_into()
                        .with_context(|| format!("row number {}, line number {}", i + 1, j + 1))
                })
            })
            .collect::<Result<Vec<Tile>, _>>()?;

        let inner = Inner::from_row_iterator(height, width, values);

        Ok(Self(inner))
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.0.row_iter() {
            for value in row.iter() {
                write!(f, "{}", value)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
