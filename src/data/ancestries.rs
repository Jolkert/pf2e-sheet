use crate::{
	data::{Feature, Meta, Prerequisite},
	stats::AttributeSet,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Ancestry {
	pub meta: Meta,
	pub size: Vec<SizeClass>,
	pub speed: Speed,
	pub base_hp: u8,
	pub at_boost: Vec<AttributeSet>,
	pub at_flaw: Vec<AttributeSet>,
	pub base_langs: Vec<String>,
	pub rec_langs: Vec<String>,
	pub features: Vec<Feature>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Heritage {
	meta: Meta,
	prereqs: Vec<Prerequisite>,
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

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct Speed {
	walk: u8,
	burrow: u8,
	climb: u8,
	fly: u8,
	swim: u8,
}

#[test]
fn test_all_serde() {
	let files =
		std::fs::read_dir("./data/ancestries").expect("Couldn't find 'ancestries' directory!");

	for file in files.flatten() {
		let path = file.path();
		if path.extension().is_some_and(|ext| ext == "ron") {
			let file_str = std::fs::read_to_string(&path).unwrap_or_else(|err| {
				panic!("Failed read file {}: {err}", path.display());
			});
			let _: Ancestry = ron::from_str(&file_str).unwrap_or_else(|err| {
				panic!("Failed to deserialize file {}: {err}", path.display())
			});
		}
	}
}
