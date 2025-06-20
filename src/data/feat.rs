use crate::{
	data::{Feature, Meta},
	stats::{Attribute, Proficiency, Stat},
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Feat {
	pub meta: Meta,
	pub level: u8,
	pub prereqs: Vec<Vec<Prerequisite>>,
	pub features: Vec<Feature>,
}

// maybe later clippy -morgan 2025-06-19
#[allow(clippy::new_without_default)]
impl Feat {
	pub fn new() -> Self {
		Feat {
			meta: Meta {
				id: String::new(),
				traits: Vec::new(),
				flavor: None,
				legacy: false,
			},
			level: 1,
			prereqs: Vec::new(),
			features: Vec::new(),
		}
	}
}

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Prerequisite {
	Proficiency(Stat, Proficiency),
	WeaponProficiency(String, Proficiency),
	AbstractProficiency(String, Proficiency),
	Heritage(String),
	Attribute(Attribute, i8),
	Feat(String),
	Subclass(String),
	Flavor(String),
	Trait(String),
	Spell(String),
	Action(String),
}
