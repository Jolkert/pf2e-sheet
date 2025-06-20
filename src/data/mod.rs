// Character struct implementation
// -Alex 2025-06-03

// Todo: Honglos Hüberhaus

mod ancestries;
mod armor;
mod character;
mod classes;
mod feat;
mod features;
mod weapons;

pub use ancestries::*;
pub use armor::*;
pub use character::*;
pub use classes::*;
pub use feat::*;
pub use features::*;
pub use weapons::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Meta {
	pub id: String,
	pub traits: Vec<String>,
	pub flavor: Option<String>,
	pub legacy: bool,
}
