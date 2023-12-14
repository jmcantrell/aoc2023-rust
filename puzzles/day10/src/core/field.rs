use anyhow::{bail, ensure, Context};

use std::collections::VecDeque;
use std::convert::TryFrom;

use super::{Grid, Location, Tile, DIRECTIONS};

#[derive(Debug, Clone)]
pub struct Field {
    pub start: Location,
    pub grid: Grid<Tile>,
}

impl Field {
    pub fn loop_path(&self) -> Vec<Location> {
        let mut path = Vec::new();

        let direction = self.grid.get(self.start).unwrap().edges().next().unwrap();
        let mut frontier = VecDeque::from([(self.start, direction)]);

        while let Some((location, direction)) = frontier.pop_front() {
            path.push(location);

            let (adjacent_location, adjacent_tile) =
                self.grid.neighbor(location, direction).unwrap();

            if adjacent_location == self.start {
                break;
            }

            let next_direction = adjacent_tile
                .edges()
                .find(|&other_direction| other_direction.opposite() != direction)
                .unwrap();

            frontier.push_back((adjacent_location, next_direction));
        }

        path
    }
}

impl TryFrom<&str> for Field {
    type Error = anyhow::Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let width = input.lines().next().context("empty input")?.len();

        let mut start: Option<Location> = None;
        let mut values: Vec<Tile> = Vec::new();

        for (i, line) in input.lines().enumerate() {
            ensure!(
                line.len() == width,
                "expected row number {} to have {} tiles, but it had {}",
                i + 1,
                width,
                line.len()
            );

            for (j, c) in line.chars().enumerate() {
                values.push(if c == 'S' {
                    start = Some((i, j));
                    Tile::Empty // Wait until other tiles are in place.
                } else {
                    c.try_into()
                        .with_context(|| format!("row number {}, column number {}", i + 1, j + 1))?
                })
            }
        }

        let height = values.len() / width;

        let mut grid = Grid::from_iter(height, width, values);

        if let Some(start) = start {
            grid[start] = DIRECTIONS
                .map(|direction| {
                    grid.neighbor(start, direction)
                        .map(|(_, tile)| tile.has_edge(direction.opposite()))
                        .unwrap_or_default()
                })
                .try_into()?;

            Ok(Self { start, grid })
        } else {
            bail!("no start tile found");
        }
    }
}
