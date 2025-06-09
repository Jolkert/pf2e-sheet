use std::collections::HashMap;

use saikoro::evaluation::SymbolTable;

use crate::{
	data::{Ancestry, Class, Heritage, Identifier},
	stats::{Attribute, Attributes, Proficiency, Stat},
};

#[derive(Debug, Clone)]
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

		// TODO: we should probably properly handle this error somehow
		// -morgan 2025-06-08
		let _ = self.roll_modifiers.insert(name, &roll_str);
	}

	fn max_hp(&self) -> u16 {
		self.ancestry.base_hp as u16
			+ (self.level as u16)
				* (self.class.hp as u16 + self.attributes.constitution.bonus as u16)
	}

	fn hurt(&mut self, dmg_amount: u16, dmg_type: String, crit: bool) {
		let prescaler = match self.damage_types.get(&dmg_type) {
			Some(DamageScale::Vulnerable) => 2.0,
			Some(DamageScale::Resistant) => 0.5,
			Some(DamageScale::Immune) => 0.0,
			None => 1.0,
		};

		let total_damage = (prescaler * dmg_amount as f32).floor() as u16;
		if total_damage > 0 {
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
			} else if total_damage <= self.temp_hp {
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

		if self.conditions.contains_key("Dying") {
			self.conditions.insert(String::from("Dying"), 0);

			let wounded = self.conditions.get("Wounded").copied().unwrap_or_default();

			self.conditions.insert(String::from("Wounded"), wounded + 1);
		}
	}
}

#[derive(Debug, Clone)]
pub struct Background {
	id: Identifier,
	fuck_you_morgan: Vec<Attribute>,
	free_attributes: u8,
	skills: Vec<Stat>,
	feats: Vec<Feat>,
	summary: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Feat {
	pub id: Identifier,
	pub text: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Condition {
	variant: ConditionType,
	level: u8,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ConditionType {
	Blinded,
	Broken,
	Clumsy,
	Concealed,
	Confused,
	Controlled,
	Dazzled,
	Deafened,
	Doomed,
	Drained,
	Dying,
	Encumbered,
	Enfeebled,
	Fascinated,
	Fatigued,
	Fleeing,
	Friendly,
	Frightened,
	Grabbed,
	Helpful,
	Hidden,
	Hostile,
	Immboilized,
	Indifferent,
	Invisible,
	Observed,
	OffGuard,
	Paralyzed,
	Immunity(String),
	Resistance(String),
	Weakness(String),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DamageScale {
	Vulnerable,
	Resistant,
	Immune,
}

#[derive(Debug, Clone)]
pub struct Proficiencies {
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

#[derive(Debug, Clone, Copy)]
pub struct Wealth {
	pp: u16,
	gp: u16,
	sp: u16,
	cp: u16,
}
