use crate::{
	data::{Condition, Meta, Weapon},
	stats::{Proficiency, Stat},
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Feature {
	Attack(Weapon),
	Flavor(Meta),
	Action(Action),
	Circumstance(Circumstances),
	Feat(String),
	Condition(Condition, String),
	Language(Option<String>),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Action {
	meta: Meta,
	actions: u8,
	reaction_trigger: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Circumstances {
	meta: Meta,
	circumstances: Vec<CircumstanceBonus>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CircumstanceBonus {
	pub stats: Option<Vec<Stat>>,
	pub proficiency: Option<Proficiency>,
	pub circumstance: Option<Vec<String>>,
	pub bonus: String,
}
