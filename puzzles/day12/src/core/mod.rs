pub type Springs = Vec<Spring>;
pub type Groups = Vec<usize>;
pub type Record = (Springs, Groups);

pub mod spring;
pub use spring::*;
