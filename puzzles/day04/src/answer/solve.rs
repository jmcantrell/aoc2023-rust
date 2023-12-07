use crate::core::Score;

use super::{Parsed1, Parsed2};

pub type Solution1 = Score;
pub type Solution2 = usize;

pub fn solve1(cards: &Parsed1) -> anyhow::Result<Solution1> {
    Ok(cards.iter().map(|card| card.score()).sum())
}

pub fn solve2(cards: &Parsed2) -> anyhow::Result<Solution2> {
    let mut count = 0;
    let mut queue: Vec<_> = cards.iter().enumerate().collect();

    while let Some((i, card)) = queue.pop() {
        count += 1;

        let match_count = card.matching_numbers().count();

        cards
            .iter()
            .enumerate()
            .skip(i + 1)
            .take(match_count)
            .for_each(|other| queue.push(other))
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn solve1() -> anyhow::Result<()> {
        assert_eq!(super::solve1(&parse1(INPUT)?)?, 13);
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        assert_eq!(super::solve2(&parse2(INPUT)?)?, 30);
        Ok(())
    }
}
