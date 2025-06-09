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
	pub fn attribute(&self) -> Attribute {
		match self {
			Stat::Acrobatics => Attribute::Dex,
			Stat::Arcana => Attribute::Int,
			Stat::Athletics => Attribute::Str,
			Stat::Crafting => Attribute::Int,
			Stat::Deception => Attribute::Cha,
			Stat::Diplomacy => Attribute::Cha,
			Stat::Fortitude => Attribute::Con,
			Stat::Intimidation => Attribute::Cha,
			Stat::Lore(_) => Attribute::Int,
			Stat::Medicine => Attribute::Wis,
			Stat::Nature => Attribute::Wis,
			Stat::Occultism => Attribute::Int,
			Stat::Perception => Attribute::Wis,
			Stat::Performance => Attribute::Cha,
			Stat::Reflex => Attribute::Dex,
			Stat::Religion => Attribute::Wis,
			Stat::Society => Attribute::Int,
			Stat::Stealth => Attribute::Dex,
			Stat::Survival => Attribute::Wis,
			Stat::Thievery => Attribute::Dex,
			Stat::Will => Attribute::Wis,
		}
	}
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct AttributeValue {
	pub bonus: i8,
	pub partial: bool,
}

impl std::ops::Add<i8> for AttributeValue {
	fn add(self, _rhs: i8) -> Self::Output {
		todo!()
	}

	type Output = Self;
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Attribute {
	Free = 0xff,
	Str = 0x01,
	Dex = 0x02,
	Con = 0x04,
	Int = 0x08,
	Wis = 0x10,
	Cha = 0x20,
}

impl Attribute {
	pub fn to_string(attribute: Attribute) -> Option<String> {
		match attribute {
			Attribute::Str => Some("str".to_string()),
			Attribute::Dex => Some("dex".to_string()),
			Attribute::Con => Some("con".to_string()),
			Attribute::Int => Some("int".to_string()),
			Attribute::Wis => Some("wis".to_string()),
			Attribute::Cha => Some("cha".to_string()),
			_ => None,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
