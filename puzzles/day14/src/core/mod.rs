pub type Location = (usize, usize);

pub mod direction;
pub use direction::*;

pub mod tile;
pub use tile::*;

pub mod platform;
pub use platform::*;
