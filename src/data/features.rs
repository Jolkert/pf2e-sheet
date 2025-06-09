use crate::{
	data::{Condition, Feat, Identifier, Weapon},
	stats::Stat,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Feature {
	Attack(Weapon),
	Flavor(Flavor),
	Action(Action),
	Circumstance(Circumstance),
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
	flavor: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Circumstance {
	id: Identifier,
	bonuses: Vec<CircumstanceBonus>,
	flavor: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CircumstanceBonus {
	stat: Stat,
	circumstance: Vec<String>,
	bonus: i8,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Flavor {
	id: Identifier,
	flavor: String,
}
