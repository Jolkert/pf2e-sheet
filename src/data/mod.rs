// Character struct implementation
// -Alex 2025-06-03

// Todo: Honglos Hüberhaus

mod ancestries;
mod character;
mod classes;
mod features;
mod weapons;

pub use ancestries::*;
pub use character::*;
pub use classes::*;
pub use features::*;
pub use weapons::*;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Identifier {
	pub id: String,
	pub traits: Vec<String>,
}
