use std::collections::{
    hash_map::{DefaultHasher, Entry},
    HashMap,
};
use std::hash::{Hash, Hasher};

use crate::core::Direction;

use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(platform: &Parsed1) -> anyhow::Result<Solution1> {
    Ok(platform.tilt(&Direction::North).total_load())
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
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
    let mut directions = directions_ccw.into_iter().cycle().peekable();

    let mut current = calculate_hash(platform);
    let mut platforms = HashMap::from([(current, platform.clone())]);
    let mut graph: HashMap<(u64, Direction), (usize, u64)> = HashMap::new();

    let (loop_start, loop_end) = (0..num_iterations)
        .find_map(
            |i| match graph.entry((current, *directions.peek().unwrap())) {
                Entry::Occupied(entry) => Some((entry.get().0, i)),
                Entry::Vacant(entry) => {
                    let platform = platforms[&current].tilt(&directions.next().unwrap());
                    current = calculate_hash(&platform);
                    platforms.insert(current, platform);
                    entry.insert((i, current));
                    None
                }
            },
        )
        .unwrap();

    let remaining_iterations = (num_iterations - loop_start) % (loop_end - loop_start);

    for _ in 0..remaining_iterations {
        current = graph.get(&(current, directions.next().unwrap())).unwrap().1;
    }

    Ok(platforms[&current].total_load())
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
