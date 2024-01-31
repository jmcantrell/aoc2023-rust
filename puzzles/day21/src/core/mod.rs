pub type Location<T> = (T, T);
pub type Grid = nalgebra::DMatrix<Tile>;

pub mod direction;
pub use direction::*;

pub mod tile;
pub use tile::*;

pub mod graph;
pub use graph::*;

pub mod map;
pub use map::*;
