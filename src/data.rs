// Character struct implementation
// -Alex 2025-06-03

// Todo: Honglos Hüberhaus

use std::collections::HashMap;

use saikoro::{evaluation::SymbolTable, *};
use serde::{Deserialize, Serialize};

use crate::{ancestries::*, fluff::*, stats::*};

pub struct Character {
	level: u8,

	hero_points: u8,

	ancestry: Ancestry,
	heritage: Heritage,
	background: Background,
	class: Class,

	attributes: Attributes,

	stats: HashMap<String, Stat>,

	roll_modifiers: SymbolTable,

	damage: u16,
	temp_hp: u16,

	conditions: HashMap<String, u8>,
	damage_types: HashMap<String, DamageScale>,

	languages: Vec<String>,

	proficiencies: Proficiencies,

	feats: Vec<Feat>,

	wealth: Wealth,
}

/*impl Default for Character {
	fn default() -> Self {

		let mut character = Character {
			level: 1,
			hero_points: 1,
			ancestry: Ancestry::default(),
			heritage: Heritage::default(),
			background: Background::default(),
			class: Class::default(),
			attributes: Attributes::default(),
			stats: HashMap::<String, Stat>::new(),
			roll_modifiers: SymbolTable::new(),
			damage: 0_u16,
			temp_hp: 0_u16,
			conditions: HashMap::<String, u8>::new(),
			damage_types: HashMap::<String, DamageScale>::new(),
			languages: Vec::<String>::new(),
			proficiencies: Proficiencies::default(),
			feats: Vec::<Feat>::new(),
			wealth: Wealth::default(),
		};

		character.insert_stat("Acrobatics",     Stat::new(Attribute::Dex, Proficiency::Untrained));
		character.insert_stat("Arcana",         Stat::new(Attribute::Int, Proficiency::Untrained));
		character.insert_stat("Athletics",      Stat::new(Attribute::Str, Proficiency::Untrained));
		character.insert_stat("Crafting",       Stat::new(Attribute::Int, Proficiency::Untrained));
		character.insert_stat("Deception",      Stat::new(Attribute::Cha, Proficiency::Untrained));
		character.insert_stat("Diplomacy",      Stat::new(Attribute::Cha, Proficiency::Untrained));
		character.insert_stat("Intimidation",   Stat::new(Attribute::Cha, Proficiency::Untrained));
		character.insert_stat("Medicine",       Stat::new(Attribute::Wis, Proficiency::Untrained));
		character.insert_stat("Nature",         Stat::new(Attribute::Wis, Proficiency::Untrained));
		character.insert_stat("Occultism",      Stat::new(Attribute::Int, Proficiency::Untrained));
		character.insert_stat("Perception",     Stat::new(Attribute::Wis, Proficiency::Untrained));
		character.insert_stat("Performance",    Stat::new(Attribute::Cha, Proficiency::Untrained));
		character.insert_stat("Religion",       Stat::new(Attribute::Wis, Proficiency::Untrained));
		character.insert_stat("Society",        Stat::new(Attribute::Int, Proficiency::Untrained));
		character.insert_stat("Stealth",        Stat::new(Attribute::Dex, Proficiency::Untrained));
		character.insert_stat("Survival",       Stat::new(Attribute::Wis, Proficiency::Untrained));
		character.insert_stat("Thievery",       Stat::new(Attribute::Dex, Proficiency::Untrained));
		character.insert_stat("Fortitude",      Stat::new(Attribute::Con, Proficiency::Untrained));
		character.insert_stat("Reflex",         Stat::new(Attribute::Dex, Proficiency::Untrained));
		character.insert_stat("Will",           Stat::new(Attribute::Wis, Proficiency::Untrained));

		character

	}
}*/

impl Character {
	fn insert_stat(&mut self, name: &str, stat_val: Stat) {
		let mut roll_str = Attribute::to_string(stat_val.attribute).unwrap();
		roll_str.insert(0, '{');
		roll_str.push('}');

		let proficiency_bonus = stat_val.proficiency.bonus();

		if proficiency_bonus > 0 {
			roll_str.push_str(&format!(" + {{level}} + {proficiency_bonus}"));
		}

		self.stats.insert(name.to_string(), stat_val);

		self.roll_modifiers.insert(name, &roll_str);
	}

	fn max_hp(&self) -> u16 {
		return self.ancestry.base_hp as u16
			+ (self.level as u16)
				* (self.class.hp as u16 + self.attributes.constitution.bonus as u16);
	}

	fn hurt(&mut self, dmg_amount: u16, dmg_type: String, crit: bool) {
		let prescaler: f32;

		if let Some(dmg_scale) = self.damage_types.get(&dmg_type) {
			prescaler = match dmg_scale {
				DamageScale::Vulnerable => 2.0,
				DamageScale::Resistant => 0.5,
				DamageScale::Immune => 0.0,
			}
		} else {
			prescaler = 1.0;
		}

		let total_damage = (prescaler * dmg_amount as f32).floor() as u16;

		if total_damage == 0 {
			return;
		};

		if let Some(dying) = self.conditions.get("Dying") {
			if crit {
				self.conditions.insert(String::from("Dying"), dying + 2);
			} else {
				self.conditions.insert(String::from("Dying"), dying + 1);
			}

			let doomed = self.conditions.get("Doomed").unwrap_or(&0_u8);
			let wounded = self.conditions.get("Wounded").unwrap_or(&0_u8);

			if self.conditions["Dying"] >= (4 - doomed - wounded) {
				self.conditions.insert(String::from("Dying"), 0);
				self.conditions.insert(String::from("Dead"), 1);
				self.conditions.insert(String::from("Doomed"), 0);
			}
		} else {
			if total_damage <= self.temp_hp {
				self.temp_hp -= total_damage;
			} else if total_damage < self.max_hp() + self.damage {
				self.temp_hp = 0;
				self.damage += total_damage;
			} else {
				self.damage = self.max_hp();
				self.conditions.insert(String::from("Dying"), 1);
			}
		}
	}

	fn heal(&mut self, heal_amount: u16) {
		if heal_amount <= self.damage {
			self.damage -= heal_amount;
		} else {
			self.damage = 0;
		}

		if let Some(_) = self.conditions.get("Dying") {
			self.conditions.insert(String::from("Dying"), 0);

			let wounded = self.conditions.get("Wounded").unwrap_or(&0_u8);

			self.conditions.insert(String::from("Wounded"), wounded + 1);
		}
	}
}

enum DamageScale {
	Vulnerable,
	Resistant,
	Immune,
}

struct Class {
	id: Identifier,
	key_attribute: Attribute,
	hp: u8,
	skills: Vec<(Stat, Proficiency)>,
	bonus_skills: u8,
	proficiencies: Proficiencies,
	subclasses: Vec<Subclass>,
}

struct Subclass {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Feat {
	pub id: Identifier,
	pub text: String,
}

struct Condition {
	id: String,
	value: i8,
}

struct Background {
	id: Identifier,
	fuck_you_morgan: Vec<Attribute>,
	free_attributes: u8,
	skills: Vec<Stat>,
	feats: Vec<Feat>,
	summary: String,
}

struct Heritage {
	id: Identifier,
	features: Vec<Feature>,
}

struct Proficiencies {
	unarmored: Proficiency,
	light: Proficiency,
	medium: Proficiency,
	heavy: Proficiency,
	unarmed: Proficiency,
	simple: Proficiency,
	martial: Proficiency,
	advanced: Proficiency,
	other: Proficiency,
}

struct Wealth {
	pp: u16,
	gp: u16,
	sp: u16,
	cp: u16,
}
