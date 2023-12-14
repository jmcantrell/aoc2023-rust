use anyhow::Context;

use std::convert::TryFrom;

use super::Value;

type Inner = Vec<Value>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct History(Inner);

fn differences(values: Vec<Value>) -> impl Iterator<Item = Vec<Value>> {
    let mut current = values;

    std::iter::from_fn(move || {
        if current.iter().all(|&value| value == 0) {
            return None;
        }

        current = current
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect();

        Some(current.clone())
    })
}

fn extrapolate(values: Vec<Value>) -> impl Iterator<Item = Value> {
    let mut current = values.last().cloned().unwrap();

    let mut latest_values: Vec<_> = differences(values)
        .map(|mut values| values.pop().unwrap())
        .collect();

    latest_values.reverse();

    std::iter::from_fn(move || {
        for i in 0..latest_values.len() - 1 {
            latest_values[i + 1] += latest_values[i];
        }

        current += latest_values.last().cloned().unwrap();

        Some(current)
    })
}

impl History {
    pub fn extrapolate_next(&self) -> impl Iterator<Item = Value> {
        extrapolate(self.0.clone())
    }

    pub fn extrapolate_prev(&self) -> impl Iterator<Item = Value> {
        let mut values = self.0.clone();
        values.reverse();
        extrapolate(values)
    }
}

impl From<Inner> for History {
    fn from(values: Inner) -> Self {
        Self(values)
    }
}

impl TryFrom<&str> for History {
    type Error = anyhow::Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        Ok(input
            .split_whitespace()
            .enumerate()
            .map(|(i, s)| s.parse().with_context(|| format!("value number {}", i + 1)))
            .collect::<Result<Vec<_>, _>>()?
            .into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_differences() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(
                    differences(Vec::from(&$input)).collect::<Vec<_>>(),
                    Vec::from($expected)
                );
            };
        }

        test!(
            [0, 3, 6, 9, 12, 15],
            [vec![3, 3, 3, 3, 3], vec![0, 0, 0, 0]]
        );

        test!(
            [1, 3, 6, 10, 15, 21],
            [vec![2, 3, 4, 5, 6], vec![1, 1, 1, 1], vec![0, 0, 0]]
        );
    }

    #[test]
    fn test_history_extrapolate_next() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(
                    History::from(Vec::from($input))
                        .extrapolate_next()
                        .take($expected.len())
                        .collect::<Vec<_>>(),
                    Vec::from($expected)
                );
            };
        }

        test!([0, 3, 6, 9, 12, 15], [18, 21, 24, 27]);

        test!([1, 3, 6, 10, 15, 21], [28, 36, 45, 55]);

        test!([10, 13, 16, 21, 30, 45], [68, 101, 146, 205]);
    }

    #[test]
    fn test_history_extrapolate_prev() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(
                    History::from(Vec::from($input))
                        .extrapolate_prev()
                        .take($expected.len())
                        .collect::<Vec<_>>(),
                    Vec::from($expected)
                );
            };
        }

        test!([0, 3, 6, 9, 12, 15], [-3, -6, -9, -12]);

        test!([1, 3, 6, 10, 15, 21], [0, 0, 1, 3]);

        test!([10, 13, 16, 21, 30, 45], [5, -4, -19, -42]);
    }
}
