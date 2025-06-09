use crate::{
	data::{Identifier, Proficiencies},
	stats::{Attribute, Proficiency, Stat},
};

#[derive(Debug, Clone)]
pub struct Class {
	pub id: Identifier,
	pub key_attribute: Attribute,
	pub hp: u8,
	pub skills: Vec<(Stat, Proficiency)>,
	pub bonus_skills: u8,
	pub proficiencies: Proficiencies,
	pub subclasses: Vec<Subclass>,
}

#[derive(Debug, Clone)]
pub struct Subclass {}
