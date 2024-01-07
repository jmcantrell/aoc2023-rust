use crate::core::{Category, Destination, Operator, Rating, Status};

use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

const START: &str = "in";

pub fn solve1((workflows, parts): &Parsed1) -> anyhow::Result<Solution1> {
    Ok(parts
        .iter()
        .filter_map(|part| {
            let mut current = START;

            loop {
                let workflow = &workflows[current];

                let destination = workflow
                    .rules
                    .iter()
                    .find_map(|rule| rule.test(part))
                    .unwrap_or(workflow.fallback.clone());

                match destination {
                    Destination::Workflow(next) => {
                        current = next;
                        continue;
                    }
                    Destination::Status(status) => {
                        break match status {
                            Status::Accepted => Some(part.total_rating()),
                            _ => None,
                        };
                    }
                }
            }
        })
        .sum())
}

pub fn solve2((workflows, _): &Parsed2) -> anyhow::Result<Solution2> {
    type RatingMinMax = [Rating; 2];

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Extents {
        extremely_cool_looking: RatingMinMax,
        musical: RatingMinMax,
        aerodynamic: RatingMinMax,
        shiny: RatingMinMax,
    }

    impl Extents {
        fn get_mut(&mut self, category: Category) -> &mut RatingMinMax {
            match category {
                Category::ExtremelyCoolLooking => &mut self.extremely_cool_looking,
                Category::Musical => &mut self.musical,
                Category::Aerodynamic => &mut self.aerodynamic,
                Category::Shiny => &mut self.shiny,
            }
        }

        fn len(&self) -> usize {
            fn len(rating_min_max: RatingMinMax) -> usize {
                rating_min_max[1] - rating_min_max[0] + 1
            }

            len(self.extremely_cool_looking)
                * len(self.musical)
                * len(self.aerodynamic)
                * len(self.shiny)
        }
    }

    impl Default for Extents {
        fn default() -> Self {
            let range = [1, 4000];

            Self {
                extremely_cool_looking: range,
                musical: range,
                aerodynamic: range,
                shiny: range,
            }
        }
    }

    let mut accepted_extents = Vec::new();
    let mut frontier = vec![(Destination::Workflow(START), 0usize, Extents::default())];

    while let Some((destination, rule_index, extents)) = frontier.pop() {
        if let Destination::Status(status) = destination {
            if status == Status::Accepted {
                accepted_extents.push(extents);
            }
            continue;
        }

        let workflow_label = destination.workflow().unwrap();
        let workflow = &workflows[workflow_label];
        let rule = &workflow.rules[rule_index];

        let mut extents_if_true = extents;
        let [min_if_true, max_if_true] = extents_if_true.get_mut(rule.condition.category);

        let mut extents_if_false = extents;
        let [min_if_false, max_if_false] = extents_if_false.get_mut(rule.condition.category);

        match rule.condition.operator {
            Operator::LessThan => {
                *min_if_false = rule.condition.rating;
                *max_if_true = rule.condition.rating.saturating_sub(1);
            }
            Operator::GreaterThan => {
                *min_if_true = rule.condition.rating.saturating_add(1);
                *max_if_false = rule.condition.rating;
            }
        }

        frontier.push((rule.if_true.clone(), 0, extents_if_true));

        if rule_index == workflow.rules.len() - 1 {
            frontier.push((workflow.fallback.clone(), 0, extents_if_false));
        } else {
            frontier.push((destination, rule_index + 1, extents_if_false));
        }
    }

    Ok(accepted_extents
        .into_iter()
        .map(|extents| extents.len())
        .sum::<usize>())
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    use super::*;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn test_solve1() -> anyhow::Result<()> {
        assert_eq!(solve1(&parse1(INPUT)?)?, 19_114);
        Ok(())
    }

    #[test]
    fn test_solve2() -> anyhow::Result<()> {
        assert_eq!(solve2(&parse2(INPUT)?)?, 167_409_079_868_000);
        Ok(())
    }
}
