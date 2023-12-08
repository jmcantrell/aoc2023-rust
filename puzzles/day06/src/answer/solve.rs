use crate::core::Race;

use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

fn partitions(n: usize) -> impl Iterator<Item = (usize, usize)> {
    (1..n).map(move |a| (a, n - a))
}

fn count_ways_to_win(race: &Race) -> usize {
    partitions(race.time)
        .map(|(hold, travel)| hold * travel)
        .filter(|&distance| distance > race.distance)
        .count()
}

pub fn solve1(races: &Parsed1) -> anyhow::Result<Solution1> {
    Ok(races.iter().map(count_ways_to_win).product())
}

pub fn solve2(race: &Parsed2) -> anyhow::Result<Solution2> {
    Ok(count_ways_to_win(race))
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn solve1() -> anyhow::Result<()> {
        assert_eq!(super::solve1(&parse1(INPUT)?)?, 4 * 8 * 9);
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        assert_eq!(super::solve2(&parse2(INPUT)?)?, 71503);
        Ok(())
    }
}
