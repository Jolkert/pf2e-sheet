use crate::{
	data::{Condition, Feat, Identifier, Weapon},
	stats::{Proficiency, Stat},
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Feature {
	Attack(Weapon),
	Flavor(Flavor),
	Action(Action),
	Circumstance(Circumstances),
	Feat(Feat),
	Condition(Condition, u8),
	Language,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Action {
	id: Identifier,
	actions: u8,
	reaction: bool,
	trigger: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Circumstances {
	id: Identifier,
	circumstances: Vec<CircumstanceBonus>,
	flavor: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CircumstanceBonus {
	pub stats: Option<Vec<Stat>>,
	pub proficiency: Option<Proficiency>,
	pub circumstance: Option<Vec<String>>,
	pub bonus: i8,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Flavor {
	id: Identifier,
	flavor: String,
}
