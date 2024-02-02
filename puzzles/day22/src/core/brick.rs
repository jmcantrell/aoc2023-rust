use anyhow::Context;

use std::cmp::max;

use super::{Location, Range};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Brick {
    pub x: Range,
    pub y: Range,
    pub z: Range,
}

impl Brick {
    pub fn new(start: Location, end: Location) -> Self {
        Self {
            x: (start.x, end.x).into(),
            y: (start.y, end.y).into(),
            z: (start.z, end.z).into(),
        }
    }

    pub fn start(&self) -> Location {
        Location::new(self.x.start, self.y.start, self.z.start)
    }

    pub fn end(&self) -> Location {
        Location::new(self.x.end, self.y.end, self.z.end)
    }

    pub fn size(&self) -> usize {
        max(max(self.x.size(), self.y.size()), self.z.size())
    }

    pub fn contains(&self, location: Location) -> bool {
        self.x.contains(location.x) && self.y.contains(location.y) && self.z.contains(location.z)
    }

    pub fn locations(&self) -> impl Iterator<Item = Location> {
        let mut xs = self.x.iter().cycle();
        let mut ys = self.y.iter().cycle();
        let mut zs = self.z.iter().cycle();

        std::iter::from_fn(move || {
            Some(Location::new(
                xs.next().unwrap(),
                ys.next().unwrap(),
                zs.next().unwrap(),
            ))
        })
        .take(self.size())
    }

    fn iter_xy(&self) -> impl Iterator<Item = (usize, usize)> {
        let len = if self.z.size() == 1 {
            max(self.x.size(), self.y.size())
        } else {
            1
        };

        let iter_x = self.x.iter().cycle();
        let iter_y = self.y.iter().cycle();

        iter_x.zip(iter_y).take(len)
    }

    pub fn locations_top(&self) -> impl Iterator<Item = Location> + '_ {
        self.iter_xy().map(|(x, y)| (x, y, self.z.end).into())
    }

    pub fn locations_bottom(&self) -> impl Iterator<Item = Location> + '_ {
        self.iter_xy().map(|(x, y)| (x, y, self.z.start).into())
    }
}

impl std::convert::TryFrom<&str> for Brick {
    type Error = anyhow::Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let (left, right) = input
            .split_once('~')
            .context("expected start/end locations to be delimited by a tilde")?;

        let start = left.try_into().context("start")?;
        let end = right.try_into().context("end")?;

        Ok(Self::new(start, end))
    }
}

impl std::fmt::Display for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}~{}", self.start(), self.end())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter_xy() {
        macro_rules! test {
            ($start:expr, $end:expr, $expected:expr) => {
                assert_eq!(
                    Brick::new($start.into(), $end.into())
                        .iter_xy()
                        .collect::<Vec<_>>(),
                    $expected.into_iter().collect::<Vec<_>>()
                );
            };
        }

        for end in 0..10 {
            // Horizontal bricks span the length of the varying axis.
            test!((0, 0, 0), (end, 0, 0), (0..=end).map(|x| (x, 0)));
            test!((0, 0, 0), (0, end, 0), (0..=end).map(|y| (0, y)));

            // Vertical bricks only span a single horizontal location.
            test!((0, 0, 0), (0, 0, end), [(0, 0)]);
        }
    }
}
