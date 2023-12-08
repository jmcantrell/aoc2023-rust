use anyhow::{ensure, Context};

use aoc::Input;

use crate::core::Race;

pub type Parsed1 = Vec<Race>;
pub type Parsed2 = Race;

fn parse_entry(s: &str) -> anyhow::Result<(&str, &str)> {
    s.split_once(':')
        .context("expected name/values to be delimited by a colon")
}

fn parse_time_and_distance(s: &str) -> anyhow::Result<(&str, &str)> {
    let mut entries = s
        .lines()
        .enumerate()
        .map(|(i, s)| parse_entry(s).with_context(|| format!("entry number {}", i + 1)));

    let mut next_entry = |expected_name: &str| -> anyhow::Result<&str> {
        let (name, value) = entries
            .next()
            .with_context(|| format!("missing {} entry", expected_name.to_lowercase()))??;

        ensure!(
            name == expected_name,
            "expected entry to be named {:?}, but it was named {:?}",
            expected_name,
            name
        );

        Ok(value)
    };

    let time = next_entry("Time")?;
    let distance = next_entry("Distance")?;

    ensure!(entries.next().is_none(), "unexpected entries");

    Ok((time, distance))
}

pub fn parse1(input: Input) -> anyhow::Result<Parsed1> {
    let (time, distance) = parse_time_and_distance(input)?;

    let entry_values = |s: &str| -> anyhow::Result<Vec<usize>> {
        s.split_whitespace()
            .enumerate()
            .map(|(i, s)| s.parse().with_context(|| format!("value number {}", i + 1)))
            .collect::<Result<Vec<_>, _>>()
    };

    let times = entry_values(time).context("times")?;
    let distances = entry_values(distance).context("distances")?;

    ensure!(
        times.len() == distances.len(),
        "number of time/distance values don't match: {} != {}",
        times.len(),
        distances.len()
    );

    Ok(times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect())
}

pub fn parse2(input: Input) -> anyhow::Result<Parsed2> {
    let (time, distance) = parse_time_and_distance(input)?;

    let entry_value = |s: &str| -> anyhow::Result<usize> {
        let mut n = 0;

        for c in s.chars() {
            if c.is_whitespace() {
                continue;
            }

            let digit = c
                .to_digit(10)
                .with_context(|| format!("invalid digit: {:?}", c))?;

            n *= 10;
            n += digit as usize;
        }

        Ok(n)
    };

    let time = entry_value(time).context("time")?;
    let distance = entry_value(distance).context("distance")?;

    Ok(Race { time, distance })
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn parse1() -> anyhow::Result<()> {
        dbg!(super::parse1(INPUT)?);
        Ok(())
    }

    #[test]
    fn parse2() -> anyhow::Result<()> {
        dbg!(super::parse2(INPUT)?);
        Ok(())
    }
}
