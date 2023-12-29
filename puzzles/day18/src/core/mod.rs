use nalgebra::{Point2, Vector2};

pub type Point = Point2<isize>;
pub type Vector = Vector2<isize>;

pub mod direction;
pub use direction::*;

pub mod instruction;
pub use instruction::*;
