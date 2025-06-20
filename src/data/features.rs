use crate::{
	data::{Armor, ArmorCategory, Condition, Meta, SizeClass, Weapon, WeaponCategory},
	stats::{Proficiency, Stat},
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Feature {
	Aura(Aura),
	Attack(Weapon),
	Defense(Armor),
	Flavor(Meta),
	Action(Action),
	Circumstance(Circumstances),
	Feat(String),
	Condition(Condition, String),
	Language(Option<String>),
	Proficiency((Stat, Proficiency)),
	WeaponCategoryProficiency((WeaponCategory, Proficiency)),
	ArmorCategoryProficiency((ArmorCategory, Proficiency)),
	Speed((String, u8)),
	Spell(Spell),
	CantripChoice(CantripChoice),
	Size(Vec<SizeClass>),
	Choice((u8, Vec<Feature>)),
	Hierarchy(Vec<Feature>),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Aura {
	meta: Meta,
	range: u8,
	effects: Vec<Feature>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Spell {
	name: String,
	tradition: SpellTradition,
	innate_casts: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CantripChoice {
	tradition: SpellTradition,
	innate: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SpellTradition {
	Arcane,
	Divine,
	Occult,
	Primal,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Action {
	meta: Meta,
	actions: u8,
	frequency: Option<String>,
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
