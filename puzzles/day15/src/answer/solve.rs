use crate::core::Operation;

use super::{Parsed1, Parsed2};

type Hash = usize;

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

fn calculate_hash(s: &str) -> Hash {
    let mut current = 0;

    for c in s.chars() {
        current += c as Hash;
        current *= 17;
        current %= 256;
    }

    current
}

pub fn solve1(steps: &Parsed1) -> anyhow::Result<Solution1> {
    Ok(steps.iter().map(|&step| calculate_hash(step)).sum())
}

pub fn solve2(steps: &Parsed2) -> anyhow::Result<Solution2> {
    type LensBox<'a> = Vec<(&'a str, usize)>;

    let mut lens_boxes: Vec<LensBox> = vec![vec![]; 256];

    let find_lens = |lens_box: &LensBox, query: &str| -> Option<usize> {
        lens_box.iter().position(|&(label, _)| label == query)
    };

    for step in steps {
        let i = calculate_hash(step.label);
        let lens_box = lens_boxes.get_mut(i).unwrap();

        match step.operation {
            Operation::Remove => {
                if let Some(j) = find_lens(lens_box, step.label) {
                    lens_box.remove(j);
                }
            }
            Operation::Set(value) => {
                if let Some(j) = find_lens(lens_box, step.label) {
                    lens_box[j] = (step.label, value);
                } else {
                    lens_box.push((step.label, value));
                }
            }
        };
    }

    Ok(lens_boxes
        .into_iter()
        .enumerate()
        .flat_map(|(i, lens_box)| {
            lens_box
                .into_iter()
                .enumerate()
                .map(move |(j, (_, value))| (i + 1) * (j + 1) * value)
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    use super::*;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn test_calculate_hash() {
        assert_eq!(calculate_hash("HASH"), 52);
    }

    #[test]
    fn test_solve1() -> anyhow::Result<()> {
        assert_eq!(solve1(&parse1(INPUT)?)?, 1320);
        Ok(())
    }

    #[test]
    fn test_solve2() -> anyhow::Result<()> {
        assert_eq!(solve2(&parse2(INPUT)?)?, 145);
        Ok(())
    }
}
