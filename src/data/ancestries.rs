use std::collections::HashMap;

use crate::{
	data::{Feature, Identifier},
	stats::Attribute,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Ancestry {
	pub id: Identifier,
	pub size: Vec<SizeClass>,
	pub speed: HashMap<String, u8>,
	pub base_hp: u8,
	pub at_boost: Vec<Attribute>,
	pub at_flaw: Vec<Attribute>,
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
fn test_all_serde() {
	let files = std::fs::read_dir("./ancestries").expect("Couldn't find 'ancestries' directory!");

	for file in files {
		if let Ok(f) = file {
			let path = f.path();
			if path.extension().is_some_and(|ext| ext == "ron") {
				let path_str = path.display();
				println!("Testing {path_str}");
				let file_str = std::fs::read_to_string(path).expect("FUCK (1)");
				let _: Ancestry = ron::from_str(&file_str).expect("FUCK (2)");
			}
		}
	}
}
