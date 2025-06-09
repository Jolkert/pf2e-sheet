use std::collections::HashMap;

use crate::{
	data::{Feature, Identifier},
	stats::AttributeSet,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Ancestry {
	pub id: Identifier,
	pub size: Vec<SizeClass>,
	pub speed: HashMap<String, u8>,
	pub base_hp: u8,
	pub at_boost: AttributeSet,
	pub at_flaw: AttributeSet,
	pub base_langs: Vec<String>,
	pub rec_langs: Vec<String>,
	pub features: Vec<Feature>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Heritage {
	id: Identifier,
	features: Vec<Feature>,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum SizeClass {
	Tiny,
	Small,
	Medium,
	Large,
	Huge,
	Gargantuan,
}

#[test]
fn test_serde() {
	let file_str = std::fs::read_to_string("ancestries/nagaji.ron").expect("FUCK (1)");
	let _: Ancestry = ron::from_str(&file_str).expect("FUCK (2)");
}
