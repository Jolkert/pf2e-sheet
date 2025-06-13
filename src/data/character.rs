use std::collections::HashMap;

use saikoro::evaluation::SymbolTable;

use crate::{
	data::{Ancestry, Class, DamageType, Feature, Heritage, Meta},
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

	stats: HashMap<Stat, Proficiency>,

	roll_modifiers: SymbolTable,

	damage: u16,
	temp_hp: u16,

	conditions: HashMap<Condition, u8>,

	languages: Vec<String>,

	proficiencies: Proficiencies,

	feats: Vec<Feat>,

	wealth: Wealth,
}

impl Character {
	fn insert_stat(&mut self, stat: Stat, proficiency: Proficiency) {
		let stat_name = stat.attribute().to_string();
		let mut roll_str = stat_name.clone();
		roll_str.insert(0, '{');
		roll_str.push('}');

		let proficiency_bonus = proficiency.bonus();

		if proficiency_bonus > 0 {
			roll_str.push_str(&format!(" + {{level}} + {proficiency_bonus}"));
		}

		self.stats.insert(stat, proficiency);

		// TODO: we should probably properly handle this error somehow
		// -morgan 2025-06-08
		let _ = self.roll_modifiers.insert(&stat_name, &roll_str);
	}

	fn max_hp(&self) -> u16 {
		self.ancestry.base_hp as u16
			+ (self.level as u16)
				* (self.class.hp as u16 + self.attributes.constitution.bonus() as u16)
	}

	fn current_hp(&self) -> u16 {
		self.max_hp().saturating_sub(self.damage)
	}

	fn hurt(&mut self, damage: u16, damage_type: DamageType, crit: bool) {
		let damage = if self
			.conditions
			.contains_key(&Condition::Immunity(damage_type))
		{
			0
		} else {
			damage
				+ self
					.conditions
					.get(&Condition::Weakness(damage_type))
					.copied()
					.unwrap_or_default() as u16
				- self
					.conditions
					.get(&Condition::Resistance(damage_type))
					.copied()
					.unwrap_or_default() as u16
		};

		if damage > 0 {
			if let Some(dying) = self.conditions.get_mut(&Condition::Dying) {
				*dying += 1 + u8::from(crit);
				let dying = *dying; //lol. lmao, even -morgan 2025-06-09

				let doomed = self
					.conditions
					.get(&Condition::Doomed)
					.copied()
					.unwrap_or_default();

				let wounded = self
					.conditions
					.get(&Condition::Wounded)
					.copied()
					.unwrap_or_default();

				if dying >= u8::saturating_sub(4, doomed + wounded) {
					self.conditions.insert(Condition::Dying, 0);
					self.conditions.insert(Condition::Doomed, 0);
					self.conditions.insert(Condition::Dead, 1);
				}
			} else {
				let total_remaining_hp = (self.current_hp() + self.temp_hp).saturating_sub(damage);

				// condition is true iff damage <= temp hp (meaning subtraction is safe without
				// underflow)
				// -morgan 2025-06-09
				if total_remaining_hp >= self.current_hp() {
					self.temp_hp -= damage;
				} else {
					self.temp_hp = 0;
					self.damage += damage;
				}

				if self.current_hp() == 0 {
					self.conditions.insert(Condition::Dying, 1);
				}
			}

			self.damage = damage.min(self.max_hp());
		}
	}

	fn heal(&mut self, heal_amount: u16) {
		if heal_amount <= self.damage {
			self.damage -= heal_amount;
		} else {
			self.damage = 0;
		}

		if let Some(dying) = self.conditions.get_mut(&Condition::Dying) {
			*dying = 0;
			*self.conditions.entry(Condition::Wounded).or_default() += 1;
		}
	}
}

#[derive(Debug, Clone)]
pub struct Background {
	meta: Meta,
	fuck_you_morgan: Vec<Attribute>,
	free_attributes: u8,
	skills: Vec<Stat>,
	feats: Vec<Feat>,
	summary: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Feat {
	pub meta: Meta,
	pub level: u8,
	pub prereqs: Vec<Prerequisite>,
	pub features: Vec<Feature>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Prerequisite {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Condition {
	Blinded,
	Broken,
	Clumsy,
	Concealed,
	Confused,
	Controlled,
	Dazzled,
	Dead,
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
	Stupefied,
	Unconscious,
	Undetected,
	Unfriendly,
	Unnoticed,
	Wounded,
	Immunity(DamageType),
	Resistance(DamageType),
	Weakness(DamageType),
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
