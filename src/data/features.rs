use crate::data::{Condition, Feat, Identifier, Weapon};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum Feature {
	Attack(Weapon),
	Flavor(Flavor),
	Action(Action),
	Circumstance(Circumstance),
	Feat(Feat),
	Condition(Condition),
	Language,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Action {
	id: Identifier,
	actions: u8,
	reaction: bool,
	trigger: String,
	flavor: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Circumstance {
	id: Identifier,
	bonuses: Vec<CircumstanceBonus>,
	flavor: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct CircumstanceBonus {
	stat_name: String,
	circumstance: Vec<String>,
	bonus: i8,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Flavor {
	id: Identifier,
	flavor: String,
}
