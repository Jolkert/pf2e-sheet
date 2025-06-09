#[derive(Debug, Clone, Copy)]
pub struct Stat {
	pub attribute: Attribute,
	pub proficiency: Proficiency,
}

impl Stat {
	pub fn new(attribute: Attribute, proficiency: Proficiency) -> Self {
		Stat {
			attribute,
			proficiency,
		}
	}
}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
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

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Attributes {
	pub strength: AttributeValue,
	pub dexterity: AttributeValue,
	pub constitution: AttributeValue,
	pub intelligence: AttributeValue,
	pub wisdom: AttributeValue,
	pub charisma: AttributeValue,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
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
