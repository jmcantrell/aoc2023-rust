pub type Location = (usize, usize);
pub type Offset = (isize, isize);

pub mod direction;
pub use direction::*;

pub mod map;
pub use map::*;
