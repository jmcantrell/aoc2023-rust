use anyhow::{ensure, Context};

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use super::{Direction, Location, Offset};

use Direction::*;

pub type HeatLoss = u32;

type Inner = nalgebra::DMatrix<HeatLoss>;

#[derive(Debug, Clone)]
pub struct Map(Inner);

fn step((row, column): Location, (vertical, horizontal): Offset) -> Option<Location> {
    row.checked_add_signed(vertical)
        .zip(column.checked_add_signed(horizontal))
}

impl Map {
    fn neighbor(&self, location: Location, direction: Direction) -> Option<(Location, HeatLoss)> {
        step(location, direction.offset())
            .and_then(|adjacent| self.0.get(adjacent).map(|value| (adjacent, *value)))
    }

    fn turns(
        &self,
        location: Location,
        direction: Direction,
    ) -> impl Iterator<Item = (Location, Direction, HeatLoss)> + '_ {
        direction.turns().into_iter().filter_map(move |direction| {
            self.neighbor(location, direction)
                .map(|(adjacent, heat_loss)| (adjacent, direction, heat_loss))
        })
    }

    pub fn minimal_heat_loss(&self, min_steps: usize, max_steps: usize) -> HeatLoss {
        // Taken (with minor changes) from:
        // https://github.com/pkusensei/adventofcode2023/blob/main/d17/src/lib.rs

        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        struct FrontierState {
            location: Location,
            direction: Direction,
            steps: usize,
            heat_loss_so_far: HeatLoss,
        }

        impl Ord for FrontierState {
            fn cmp(&self, other: &Self) -> Ordering {
                other
                    .heat_loss_so_far
                    .cmp(&self.heat_loss_so_far)
                    .then_with(|| self.location.cmp(&other.location))
            }
        }

        impl PartialOrd for FrontierState {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
        struct BestKey {
            location: Location,
            direction: Direction,
            steps: usize,
        }

        impl From<FrontierState> for BestKey {
            fn from(frontier_state: FrontierState) -> Self {
                Self {
                    location: frontier_state.location,
                    direction: frontier_state.direction,
                    steps: frontier_state.steps,
                }
            }
        }

        let start = (0, 0);
        let end = (self.0.nrows() - 1, self.0.ncols() - 1);

        let mut frontier: BinaryHeap<_> = [East, South]
            .into_iter()
            .map(|direction| FrontierState {
                location: start,
                direction,
                steps: 0,
                heat_loss_so_far: 0,
            })
            .collect();

        let mut best: HashMap<BestKey, HeatLoss> = frontier
            .iter()
            .cloned()
            .map(|frontier_state| (frontier_state.into(), 0))
            .collect();

        while let Some(state) = frontier.pop() {
            if state.location == end && state.steps >= min_steps {
                return state.heat_loss_so_far;
            }

            for (adjacent, direction, heat_loss) in self.turns(state.location, state.direction) {
                let next_state = FrontierState {
                    location: adjacent,
                    direction,
                    steps: if state.direction == direction {
                        state.steps + 1
                    } else {
                        1
                    },
                    heat_loss_so_far: state.heat_loss_so_far + heat_loss,
                };

                let next_key = next_state.into();

                if next_state.steps > max_steps
                    || (next_state.direction != state.direction && state.steps < min_steps)
                    || best.get(&next_key).is_some_and(|&best_heat_loss_so_far| {
                        best_heat_loss_so_far <= next_state.heat_loss_so_far
                    })
                {
                    continue;
                }

                frontier.push(next_state);
                best.insert(next_key, next_state.heat_loss_so_far);
            }
        }

        unreachable!()
    }
}

impl std::convert::TryFrom<&str> for Map {
    type Error = anyhow::Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let lines: Vec<_> = input.lines().collect();

        let height = lines.len();
        ensure!(height > 0, "empty input");

        let width = lines[0].len();
        ensure!(width > 0, "empty first line");

        let values = lines
            .into_iter()
            .enumerate()
            .flat_map(|(i, line)| {
                line.chars().enumerate().map(move |(j, c)| {
                    c.to_digit(10)
                        .with_context(|| format!("row number {}, column number {}", i + 1, j + 1))
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        let inner = Inner::from_row_iterator(height, width, values);

        Ok(Self(inner))
    }
}

impl std::fmt::Display for Map {
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
