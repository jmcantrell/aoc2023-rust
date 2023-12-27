pub type Location = (usize, usize);
pub type Offset = (isize, isize);

pub mod direction;
pub use direction::*;

pub mod tile;
pub use tile::*;

pub mod contraption;
pub use contraption::*;
