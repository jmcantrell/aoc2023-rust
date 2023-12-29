use super::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Instruction {
    pub direction: Direction,
    pub distance: usize,
}
