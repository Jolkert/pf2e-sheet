use enumset::EnumSet;

mod attribute_ops;
mod attribute_serde;

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
		todo!()
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

#[cfg(test)]
mod tests {
	use super::*;

	#[derive(Debug, serde::Deserialize, PartialEq, Eq)]
	struct TestAttributeContainer {
		single_attribute: AttributeSet,
		full: AttributeSet,
		explicit_seq: AttributeSet,
		questionable: AttributeSet,
	}

	#[test]
	fn attribute_deser_test() {
		let ron = r#"
		(
			single_attribute: Str,
			full: Free,
			explicit_seq: [Con, Int],
			questionable: [Dex]
		)"#;

		let expected = TestAttributeContainer {
			single_attribute: Attribute::Str.into(),
			full: AttributeSet::all(),
			explicit_seq: Attribute::Con | Attribute::Int,
			questionable: Attribute::Dex.into(),
		};

		assert_eq!(
			ron::from_str::<TestAttributeContainer>(ron).expect("oops!"),
			expected
		)
	}
}
