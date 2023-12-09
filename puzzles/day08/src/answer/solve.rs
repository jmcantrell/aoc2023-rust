use num::integer::lcm;

use std::collections::HashMap;

use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(map: &Parsed1) -> anyhow::Result<Solution1> {
    let mut camel = "AAA";

    for (steps, direction) in (0..).zip(map.iter_instructions()) {
        if camel == "ZZZ" {
            return Ok(steps);
        }
        camel = map.nodes[camel].traverse(direction);
    }

    unreachable!()
}

pub fn solve2(map: &Parsed2) -> anyhow::Result<Solution2> {
    let mut ghosts: Vec<_> = map
        .nodes
        .values()
        .map(|node| node.label)
        .filter(|label| label.ends_with('A'))
        .collect();

    let ghosts_len = ghosts.len();
    let mut steps_to_end = HashMap::new();

    for (steps, direction) in (1..).zip(map.iter_instructions()) {
        for (i, ghost) in ghosts.iter_mut().enumerate() {
            *ghost = map.nodes[ghost].traverse(direction);

            if ghost.ends_with('Z') {
                steps_to_end.insert(i, steps);

                if steps_to_end.len() == ghosts_len {
                    return Ok(steps_to_end.into_values().fold(1, lcm));
                }
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    use super::*;

    const INPUT1: Input = include_str!("../../input-test1.txt");
    const INPUT2: Input = include_str!("../../input-test2.txt");
    const INPUT3: Input = include_str!("../../input-test3.txt");

    #[test]
    fn test_solve1() -> anyhow::Result<()> {
        assert_eq!(solve1(&parse1(INPUT1)?)?, 2);
        assert_eq!(solve1(&parse1(INPUT2)?)?, 6);
        Ok(())
    }

    #[test]
    fn test_solve2() -> anyhow::Result<()> {
        assert_eq!(solve2(&parse2(INPUT3)?)?, 6);
        Ok(())
    }
}
