use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

use super::{Direction, Location};

use Direction::*;

pub trait Graph {
    type Component;

    fn start(&self) -> Location<Self::Component>;

    fn neighbor(
        &self,
        location: Location<Self::Component>,
        direction: Direction,
    ) -> Option<Location<Self::Component>>;

    fn iter_walk(&self) -> IterGraphWalk<Self>
    where
        Self: Sized,
    {
        IterGraphWalk::new(self)
    }

    fn iter_walk_reached_counts(&self) -> impl Iterator<Item = usize>
    where
        Self: Sized,
        Self::Component: Copy + Eq + Hash,
    {
        let mut this_step_parity = false;
        let mut reached_by_parity: [HashSet<Location<Self::Component>>; 2] = Default::default();

        self.iter_walk().map(move |visited| {
            let reached = &mut reached_by_parity[this_step_parity as usize];
            this_step_parity = !this_step_parity;
            reached.extend(visited);
            reached.len()
        })
    }
}

#[derive(Debug, Clone)]
pub struct IterGraphWalk<'a, G: 'a + Graph> {
    graph: &'a G,
    frontier: Vec<Location<G::Component>>,
    reached: HashSet<Location<G::Component>>,
}

impl<'a, G: 'a + Graph> IterGraphWalk<'a, G> {
    pub fn new(graph: &'a G) -> Self {
        Self {
            graph,
            frontier: vec![graph.start()],
            reached: Default::default(),
        }
    }
}

impl<'a, G: 'a + Graph> Iterator for IterGraphWalk<'a, G>
where
    <G as Graph>::Component: Copy + Eq + Hash,
{
    type Item = Vec<Location<G::Component>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut new_frontier: Vec<Location<G::Component>> = Default::default();
        let mut visited: Vec<Location<G::Component>> = Vec::with_capacity(self.frontier.len());

        while let Some(location) = self.frontier.pop() {
            visited.push(location);
            for direction in [North, East, South, West] {
                if let Some(adjacent) = self.graph.neighbor(location, direction) {
                    if self.reached.insert(adjacent) {
                        new_frontier.push(adjacent);
                    }
                }
            }
        }

        self.frontier = new_frontier;

        Some(visited)
    }
}
