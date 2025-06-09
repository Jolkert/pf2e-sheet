use crate::data::Feat;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Identifier {
	pub id: String,
	pub traits: Vec<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Feature {
	Attack(Weapon),
	Flavor(Flavor),
	Action(Action),
	Circumstance(Circumstance),
	Feat(Feat),
	Condition(Condition),
	Language,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Weapon {
	pub id: Identifier,
	pub price: u16,
	pub damage: String,
	pub damage_type: String,
	pub range: u16,
	pub bulk: u16,
	pub hands: u8,
	pub group: WeaponGroup,
	pub category: WeaponCategory,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum WeaponGroup {
	Axe,
	Bow,
	Club,
	Crossbow,
	Dart,
	Firearm,
	Flail,
	Knife,
	Pick,
	Polearm,
	Sling,
	Spear,
	Sword,
	Shield,
	Hammer,
	Brawling,
	Bomb,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum WeaponCategory {
	Advanced,
	Ammunition,
	Martial,
	Simple,
	Unarmed,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Action {
	id: Identifier,
	actions: u8,
	reaction: bool,
	trigger: String,
	flavor: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Flavor {
	id: Identifier,
	flavor: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Circumstance {
	id: Identifier,
	bonuses: Vec<CircumstanceBonus>,
	flavor: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CircumstanceBonus {
	stat_name: String,
	circumstance: Vec<String>,
	bonus: i8,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Condition {
	name: String,
	level: u8,
}
