use std::collections::{hash_map::Entry, HashMap};

use crate::core::{Direction, Platform};

use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(platform: &Parsed1) -> anyhow::Result<Solution1> {
    Ok(platform.tilt(&Direction::North).total_load())
}

pub fn solve2(platform: &Parsed2) -> anyhow::Result<Solution2> {
    let directions_ccw = [
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ];

    let num_cycles = 1_000_000_000;
    let num_iterations = num_cycles * directions_ccw.len();
    let mut transitions = directions_ccw.into_iter().cycle().peekable();

    let mut current = platform.clone();
    let mut cache: HashMap<(Platform, Direction), (usize, Platform)> = HashMap::new();

    let (start, end) = (0..num_iterations)
        .find_map(
            |end| match cache.entry((current.clone(), *transitions.peek().unwrap())) {
                Entry::Occupied(entry) => {
                    let start = entry.get().0;
                    Some((start, end))
                }
                Entry::Vacant(entry) => {
                    let next = current.tilt(&transitions.next().unwrap());
                    entry.insert((end, next.clone()));
                    current = next;
                    None
                }
            },
        )
        .unwrap_or((0, num_iterations));

    let loop_size = end - start;
    let remaining_iterations = (num_iterations - start) % loop_size;

    for _ in 0..remaining_iterations {
        let direction = transitions.next().unwrap();
        current = cache.get(&(current, direction)).unwrap().1.clone();
    }

    Ok(current.total_load())
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    use super::*;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn test_solve1() -> anyhow::Result<()> {
        assert_eq!(solve1(&parse1(INPUT)?)?, 136);
        Ok(())
    }

    #[test]
    fn test_solve2() -> anyhow::Result<()> {
        assert_eq!(solve2(&parse2(INPUT)?)?, 64);
        Ok(())
    }
}
