pub type Instructions = Vec<Direction>;

pub mod direction;
pub use direction::*;

pub mod node;
pub use node::*;

pub mod network;
pub use network::*;
