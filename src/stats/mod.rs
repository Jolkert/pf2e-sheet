use enumset::EnumSet;

mod attribute_ops;
mod attribute_serde;

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum Stat<S = String> {
	Acrobatics,
	Arcana,
	Athletics,
	Crafting,
	Deception,
	Diplomacy,
	Fortitude,
	Intimidation,
	Lore(S),
	Medicine,
	Nature,
	Occultism,
	Perception,
	Performance,
	Reflex,
	Religion,
	Society,
	Stealth,
	Survival,
	Thievery,
	Will,
}

impl Stat {
	// I just want this to look sane and i cant find a rustfmt setting that actually
	// does quite what i want here lol
	// -morgan 2025-06-12
    #[rustfmt::skip]
	pub fn attribute(&self) -> Attribute {
		match self {
			Stat::Athletics => Attribute::Str,

			Stat::Acrobatics
			| Stat::Reflex
			| Stat::Stealth
			| Stat::Thievery => Attribute::Dex,

			Stat::Fortitude => Attribute::Con,

			Stat::Arcana
			| Stat::Crafting
			| Stat::Lore(_)
			| Stat::Occultism
			| Stat::Society => Attribute::Int,

			Stat::Medicine
			| Stat::Nature
			| Stat::Perception
			| Stat::Religion
			| Stat::Survival
			| Stat::Will => Attribute::Wis,

			Stat::Deception
			| Stat::Diplomacy
			| Stat::Intimidation
			| Stat::Performance => Attribute::Cha,
		}
	}
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct AttributeValue(i8);
impl AttributeValue {
	pub fn bonus(self) -> i8 {
		if self.0 <= 4 {
			self.0
		} else {
			4 + (self.0 - 4) / 2
		}
	}
	pub fn partial_boost(self) -> bool {
		if self.0 <= 4 {
			false
		} else {
			self.0 % 2 != 0
		}
	}
}

impl std::ops::Add<i8> for AttributeValue {
	type Output = Self;

	fn add(self, rhs: i8) -> Self::Output {
		Self(self.0 + rhs)
	}
}
impl std::ops::Sub<i8> for AttributeValue {
	type Output = Self;

	fn sub(self, rhs: i8) -> Self::Output {
		self + -rhs
	}
}

#[derive(Debug, enumset::EnumSetType, serde::Serialize, serde::Deserialize)]
#[enumset(no_ops)]
#[enumset(serialize_repr = "list")]
pub enum Attribute {
	Str,
	Dex,
	Con,
	Int,
	Wis,
	Cha,
}
impl Attribute {
	const FREE: AttributeSet = AttributeSet::all();
}

impl std::ops::Deref for AttributeSet {
	type Target = EnumSet<Attribute>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl std::fmt::Display for Attribute {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let to_str = match self {
			Self::Str => "Str",
			Self::Dex => "Dex",
			Self::Con => "Con",
			Self::Int => "Int",
			Self::Wis => "Wis",
			Self::Cha => "Cha",
		};

		write!(f, "{to_str}")
	}
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Attributes {
	pub strength: AttributeValue,
	pub dexterity: AttributeValue,
	pub constitution: AttributeValue,
	pub intelligence: AttributeValue,
	pub wisdom: AttributeValue,
	pub charisma: AttributeValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttributeSet(EnumSet<Attribute>);
impl AttributeSet {
	pub const fn all() -> Self {
		Self(EnumSet::all())
	}
}

impl From<EnumSet<Attribute>> for AttributeSet {
	fn from(value: EnumSet<Attribute>) -> Self {
		Self(value)
	}
}
impl From<Attribute> for AttributeSet {
	fn from(value: Attribute) -> Self {
		Self(value.into())
	}
}

#[derive(
	Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum Proficiency {
	Untrained,
	Trained,
	Expert,
	Master,
	Legendary,
}

impl Proficiency {
	pub fn bonus(&self) -> u8 {
		match self {
			Self::Untrained => 0,
			Self::Trained => 2,
			Self::Expert => 4,
			Self::Master => 6,
			Self::Legendary => 8,
		}
	}
}
