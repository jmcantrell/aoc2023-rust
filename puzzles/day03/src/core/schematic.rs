use anyhow::Context;

use std::collections::{HashSet, VecDeque};
use std::convert::TryFrom;
use std::ops::Index;

use super::{Digit, Space, Symbol, Value};

type Location = (usize, usize);
type Direction = (isize, isize);
type Cell<'a, T> = (Location, &'a T);
type Inner = nalgebra::DMatrix<Space>;

const DIRECTIONS: [Direction; 8] = [
    (-1, -1), // Northwest
    (-1, 0),  // North
    (-1, 1),  // Northeast
    (0, -1),  // West
    (0, 1),   // East
    (1, -1),  // Southwest
    (1, 0),   // South
    (1, 1),   // Southeast
];

fn adjacent_locations((row, column): Location) -> impl Iterator<Item = Location> {
    DIRECTIONS
        .into_iter()
        .filter_map(move |(vertical, horizontal)| {
            let row = row.checked_add_signed(vertical)?;
            let column = column.checked_add_signed(horizontal)?;
            Some((row, column))
        })
}

fn digits_to_number(digits: impl IntoIterator<Item = Digit>) -> Value {
    digits.into_iter().fold(0, |mut number, digit| {
        number *= 10;
        number += digit as Value;
        number
    })
}

#[derive(Debug, Clone)]
pub struct Schematic(Inner);

impl Index<Location> for Schematic {
    type Output = Space;

    fn index(&self, (row, column): Location) -> &Self::Output {
        &self.0[(column, row)]
    }
}

impl Schematic {
    fn get(&self, (row, column): Location) -> Option<&Space> {
        self.0.get((column, row))
    }

    fn locations(&self) -> impl Iterator<Item = Location> {
        let (height, width) = self.0.shape();
        (0..height).flat_map(move |row| (0..width).map(move |column| (row, column)))
    }

    fn symbols(&self) -> impl Iterator<Item = Cell<Symbol>> {
        self.locations()
            .filter_map(|location| self[location].symbol().map(|symbol| (location, symbol)))
    }

    fn adjacent_digit_groups(&self, location: Location) -> impl Iterator<Item = Vec<Digit>> + '_ {
        let mut queue: Vec<_> = adjacent_locations(location).collect();
        let mut seen = HashSet::new();

        let width = self.0.ncols();

        std::iter::from_fn(move || {
            while let Some(location) = queue.pop() {
                if !seen.insert(location) {
                    continue;
                }

                let maybe_digit = self.get(location).and_then(|space| space.digit());

                if maybe_digit.is_none() {
                    continue;
                }

                let (row, column) = location;

                let mut group = VecDeque::from([maybe_digit.copied().unwrap()]);

                for column in (0..column).rev() {
                    let location = (row, column);
                    if let Some(digit) = self.get(location).and_then(|space| space.digit()) {
                        group.push_front(*digit);
                        seen.insert(location);
                    } else {
                        break;
                    }
                }

                for column in column + 1..width {
                    let location = (row, column);
                    if let Some(digit) = self.get(location).and_then(|space| space.digit()) {
                        group.push_back(*digit);
                        seen.insert(location);
                    } else {
                        break;
                    }
                }

                return Some(group.into_iter().collect());
            }

            None
        })
    }

    pub fn part_numbers(&self) -> impl Iterator<Item = Value> + '_ {
        self.symbols()
            .flat_map(|(location, _)| self.adjacent_digit_groups(location).map(digits_to_number))
    }

    pub fn gear_ratios(&self) -> impl Iterator<Item = Value> + '_ {
        self.symbols()
            .filter(|(_, &symbol)| symbol == '*')
            .flat_map(|(location, _)| {
                let groups: Vec<_> = self.adjacent_digit_groups(location).collect();
                (groups.len() == 2).then_some(groups.into_iter().map(digits_to_number).product())
            })
    }
}

impl TryFrom<&str> for Schematic {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> anyhow::Result<Self> {
        let mut lines = s.lines().enumerate().peekable();

        let width = lines
            .peek()
            .map(|(_, first_line)| first_line.len())
            .context("empty input")?;

        let spaces = lines
            .flat_map(|(i, s)| {
                s.chars().enumerate().map(move |(j, c)| {
                    c.try_into()
                        .with_context(|| format!("row number {}, column number {}", i + 1, j + 1))
                })
            })
            .collect::<Result<Vec<Space>, _>>()?;

        let height = spaces.len() / width;
        let inner = Inner::from_vec(height, width, spaces);

        Ok(Self(inner))
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../../input-test.txt");

    #[test]
    fn part_numbers() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(
                    super::Schematic::try_from(INPUT)
                        .unwrap()
                        .part_numbers()
                        .collect::<Vec<_>>(),
                    Vec::from($expected)
                );
            };
        }

        test!(INPUT, [35, 467, 633, 617, 592, 664, 598, 755]);
    }

    #[test]
    fn gear_ratios() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(
                    super::Schematic::try_from(INPUT)
                        .unwrap()
                        .gear_ratios()
                        .collect::<Vec<_>>(),
                    Vec::from($expected)
                );
            };
        }

        test!(INPUT, [16345, 451490]);
    }
}
