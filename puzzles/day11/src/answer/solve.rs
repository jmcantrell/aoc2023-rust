use itertools::Itertools;

use std::collections::HashSet;

use crate::core::{Image, Location};

use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

fn galaxy_distances(
    image: &Image,
    multiplier: usize,
) -> impl Iterator<Item = ((Location, Location), usize)> + '_ {
    let top = image.iter().min_by_key(|(row, _)| row).unwrap().0;
    let bottom = image.iter().max_by_key(|(row, _)| row).unwrap().0;
    let left = image.iter().min_by_key(|(_, column)| column).unwrap().1;
    let right = image.iter().max_by_key(|(_, column)| column).unwrap().1;

    let empty_rows: HashSet<_> = (top..=bottom)
        .filter(|&row| (left..=right).all(|column| !image.contains(&(row, column))))
        .collect();

    let empty_columns: HashSet<_> = (left..=right)
        .filter(|&column| (top..=bottom).all(|row| !image.contains(&(row, column))))
        .collect();

    image.iter().combinations(2).map(move |mut pair| {
        let (row1, column1) = pair.pop().copied().unwrap();
        let (row2, column2) = pair.pop().copied().unwrap();

        let top = row1.min(row2);
        let bottom = row1.max(row2);
        let left = column1.min(column2);
        let right = column1.max(column2);

        let num_empty_rows = (top..=bottom)
            .filter(|row| empty_rows.contains(row))
            .count();

        let num_empty_columns = (left..=right)
            .filter(|column| empty_columns.contains(column))
            .count();

        let distance =
            (bottom - top) + (right - left) + (num_empty_rows + num_empty_columns) * multiplier;

        (((top, left), (bottom, right)), distance)
    })
}

fn solve(image: &Image, multiplier: usize) -> Solution {
    galaxy_distances(image, multiplier)
        .map(|(_, distance)| distance)
        .sum()
}

pub fn solve1(image: &Parsed1) -> anyhow::Result<Solution1> {
    Ok(solve(image, 1))
}

pub fn solve2(image: &Parsed2) -> anyhow::Result<Solution2> {
    Ok(solve(image, 999_999))
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::parse;

    use super::*;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn test_solve() {
        let image = parse(INPUT).unwrap();
        assert_eq!(solve(&image, 1), 374);
        assert_eq!(solve(&image, 9), 1030);
        assert_eq!(solve(&image, 99), 8410);
    }
}
