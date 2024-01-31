use anyhow::{ensure, Context};

use aoc::Input;

use crate::core::{Grid, Location, MapInfinite, MapWalled, Tile};

pub type Parsed1 = MapWalled;
pub type Parsed2 = MapInfinite;

fn parse_grid(input: Input) -> anyhow::Result<(Grid, Location<usize>)> {
    let lines: Vec<_> = input.lines().collect();

    let height = lines.len();
    ensure!(height > 0, "empty input");

    let width = lines[0].len();
    ensure!(width > 0, "empty first line");

    let mut maybe_start: Option<Location<usize>> = None;
    let mut values = Vec::with_capacity(height * width);

    for (i, line) in lines.into_iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            values.push(if c == 'S' {
                maybe_start = Some((i, j));
                Tile::Garden
            } else {
                c.try_into()
                    .with_context(|| format!("row number {}, column number {}", i + 1, j + 1))?
            });
        }
    }

    let grid = Grid::from_row_iterator(height, width, values);
    let start = maybe_start.context("missing start location")?;

    Ok((grid, start))
}

pub fn parse1(input: Input) -> anyhow::Result<Parsed1> {
    let (grid, start) = parse_grid(input)?;
    Ok(MapWalled { grid, start })
}

pub fn parse2(input: Input) -> anyhow::Result<Parsed2> {
    let (grid, (row, column)) = parse_grid(input)?;
    let start = (row as isize, column as isize);
    Ok(MapInfinite { grid, start })
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use super::*;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn test_parse1() -> anyhow::Result<()> {
        dbg!(parse1(INPUT)?);
        Ok(())
    }

    #[test]
    fn test_parse2() -> anyhow::Result<()> {
        dbg!(parse2(INPUT)?);
        Ok(())
    }
}
