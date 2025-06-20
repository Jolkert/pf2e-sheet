use crate::data::Meta;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Weapon {
	pub meta: Meta,
	pub price: u16,
	pub damage: String,
	pub damage_type: DamageType,
	pub range: Option<u16>,
	pub bulk: u16,
	pub hands: u8,
	pub group: WeaponGroup,
	pub category: WeaponCategory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum WeaponCategory {
	Advanced,
	Ammunition,
	Martial,
	Simple,
	Unarmed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum DamageType {
	Bludgeoning,
	Piercing,
	Slashing,
	Acid,
	Cold,
	Electricity,
	Fire,
	Sonic,
	Spirit,
	Mental,
	// TODO: is this right? -morgan 2025-06-08
	Poison,
	Bleed,
}
