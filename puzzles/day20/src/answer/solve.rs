use anyhow::Context;
use num::integer::lcm;

use crate::core::PulseKind;

use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(config: &Parsed1) -> anyhow::Result<Solution1> {
    let mut low = 0;
    let mut high = 0;

    for pulse_history in config.iter_button_pushes("broadcaster").take(1000) {
        for (pulse, _) in pulse_history {
            match pulse {
                PulseKind::Low => low += 1,
                PulseKind::High => high += 1,
            }
        }
    }

    Ok(low * high)
}

pub fn solve2(config: &Parsed2) -> anyhow::Result<Solution2> {
    let end = "rx";

    config.outputs["broadcaster"]
        .iter()
        .map(|start| {
            config
                .slice(start, end)
                .iter_button_pushes(start)
                .position(|pulse_history| {
                    pulse_history
                        .into_iter()
                        .any(|(pulse, to)| pulse == PulseKind::Low && to == end)
                })
                .unwrap()
                + 1
        })
        .reduce(lcm)
        .context("no solution")
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::parse1;

    use super::*;

    const INPUT1: Input = include_str!("../../input-test1.txt");
    const INPUT2: Input = include_str!("../../input-test2.txt");

    #[test]
    fn test_solve1() -> anyhow::Result<()> {
        assert_eq!(solve1(&parse1(INPUT1)?)?, 32_000_000);
        assert_eq!(solve1(&parse1(INPUT2)?)?, 11_687_500);
        Ok(())
    }
}
