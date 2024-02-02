use anyhow::Context;

use std::collections::{HashMap, HashSet};

use super::{Brick, Location};

const FLOOR: usize = 1;

type Dependencies<'a> = HashMap<&'a Brick, HashSet<&'a Brick>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stack {
    pub bricks: Vec<Brick>,
    location_to_brick: HashMap<Location, usize>,
}

impl Stack {
    pub fn new(bricks: Vec<Brick>) -> Self {
        let location_to_brick: HashMap<Location, usize> = bricks
            .iter()
            .enumerate()
            .flat_map(|(i, brick)| brick.locations().map(move |location| (location, i)))
            .collect();

        Self {
            bricks,
            location_to_brick,
        }
    }

    pub fn brick_at(&self, location: &Location) -> Option<&Brick> {
        self.location_to_brick
            .get(location)
            .map(|&i| &self.bricks[i])
    }

    pub fn settle(&mut self) {
        loop {
            let mut changed = false;

            for (i, brick) in self.bricks.iter_mut().enumerate() {
                let bottom = brick.z.start;

                let lowest_z = brick
                    .locations_bottom()
                    .map(|Location { x, y, z: bottom }| {
                        (FLOOR..bottom)
                            .rev()
                            .find(|&z| self.location_to_brick.contains_key(&(x, y, z).into()))
                            .map(|z| z + 1)
                            .unwrap_or(FLOOR)
                    })
                    .max()
                    .unwrap();

                if lowest_z == bottom {
                    continue;
                }

                let offset = bottom - lowest_z;

                for location in brick.locations() {
                    self.location_to_brick.remove(&location);
                }

                brick.z -= offset;

                for location in brick.locations() {
                    self.location_to_brick.insert(location, i);
                }

                changed = true;
            }

            if !changed {
                break;
            }
        }
    }

    pub fn dependencies(&self) -> (Dependencies, Dependencies) {
        self.bricks.iter().fold(
            (HashMap::new(), HashMap::new()),
            |(mut supports, mut is_supported_by), brick| {
                let bricks_below = brick
                    .locations_bottom()
                    .filter_map(|location| self.brick_at(&location.below()))
                    .collect();

                let bricks_above = brick
                    .locations_top()
                    .filter_map(|location| self.brick_at(&location.above()))
                    .collect();

                supports.insert(brick, bricks_below);
                is_supported_by.insert(brick, bricks_above);

                (supports, is_supported_by)
            },
        )
    }
}

impl std::fmt::Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for brick in self.bricks.iter() {
            writeln!(f, "{}", brick)?;
        }
        Ok(())
    }
}

impl std::convert::TryFrom<&str> for Stack {
    type Error = anyhow::Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        Ok(Self::new(
            input
                .lines()
                .enumerate()
                .map(|(i, line)| {
                    line.try_into()
                        .with_context(|| format!("brick number {}", i + 1))
                })
                .collect::<Result<Vec<Brick>, _>>()?,
        ))
    }
}
