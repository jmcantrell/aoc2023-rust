use anyhow::{anyhow, ensure, Context};
use nalgebra::DMatrix;

use std::convert::TryFrom;

type Inner = DMatrix<bool>;

#[derive(Debug, Clone)]
pub struct Grid(Inner);

impl From<Inner> for Grid {
    fn from(inner: Inner) -> Self {
        Self(inner)
    }
}

fn diff<'a, T: 'a + PartialEq, I>(iter: I, mid: usize) -> impl Iterator<Item = (usize, usize)> + 'a
where
    I: Iterator<Item = &'a T> + Clone + ExactSizeIterator + DoubleEndedIterator + 'a,
{
    let before = iter.clone().enumerate().take(mid).rev();
    let after = iter.enumerate().skip(mid);

    before
        .zip(after)
        .filter_map(|((i, a), (j, b))| (a != b).then_some((i, j)))
}

type Start = (usize, usize);
type Shape = (usize, usize);

impl Grid {
    pub fn height(&self) -> usize {
        self.0.nrows()
    }

    pub fn width(&self) -> usize {
        self.0.ncols()
    }

    pub fn reflection_diff_counts(
        &self,
        primary: usize,
        secondary: usize,
        view: fn(usize, usize) -> (Start, Shape),
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..primary - 1).map(move |index| {
            (
                index,
                (0..secondary)
                    .map(|column| {
                        let (start, shape) = view(column, primary);
                        diff(self.0.view(start, shape).iter(), index + 1).count()
                    })
                    .sum::<usize>(),
            )
        })
    }
}

impl TryFrom<&str> for Grid {
    type Error = anyhow::Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        fn parse_value(input: char) -> anyhow::Result<bool> {
            match input {
                '#' => Ok(true),
                '.' => Ok(false),
                _ => Err(anyhow!("invalid value: {:?}", input)),
            }
        }

        let lines: Vec<_> = input.lines().enumerate().collect();

        let height = lines.len();
        let width = lines.first().context("empty input")?.1.len();

        for (i, line) in lines.iter().skip(1) {
            ensure!(
                line.len() == width,
                "expected row number {} to have {} tiles, but it had {}",
                i + 1,
                width,
                line.len()
            );
        }

        let mut values = Vec::with_capacity(height * width);

        for (i, line) in lines {
            for (j, c) in line.chars().enumerate() {
                values.push(
                    parse_value(c).with_context(|| {
                        format!("row number {}, column number {}", i + 1, j + 1)
                    })?,
                );
            }
        }

        Ok(Inner::from_row_iterator(height, width, values).into())
    }
}
