use std::marker::PhantomData;

use serde::{
	Deserialize,
	de::{VariantAccess, Visitor},
};

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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Attributes {
	pub strength: AttributeValue,
	pub dexterity: AttributeValue,
	pub constitution: AttributeValue,
	pub intelligence: AttributeValue,
	pub wisdom: AttributeValue,
	pub charisma: AttributeValue,
}

pub type AttributeSet = enumset::EnumSet<Attribute>;

#[derive(Debug, enumset::EnumSetType, serde::Serialize, serde::Deserialize)]
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

pub(crate) fn deserialize_attribute_set<'de, D>(deserializer: D) -> Result<AttributeSet, D::Error>
where
	D: serde::Deserializer<'de>,
{
	// struct AttributeVisitor;
	// impl<'de> Visitor<'de> for AttributeVisitor {
	// 	type Value = AttributeSet;
	//
	// 	fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
	// 		formatter.write_str("attribute value, set of attributes, or 'Free'")
	// 	}
	//
	// 	// fn visit_unit<E>(self) -> Result<Self::Value, E>
	// 	// where
	// 	// 	E: serde::de::Error,
	// 	// {
	// 	// 	SerAttribute::deserialize(serde::de::value::UnitDeserializer::new()).map(Into::into)
	// 	// }
	//
	// 	fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
	// 	where
	// 		A: serde::de::EnumAccess<'de>,
	// 	{
	// 		Attribute::deserialize(serde::de::value::EnumAccessDeserializer::new(data))
	// 			.map(Into::into)
	// 	}
	//
	// 	fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
	// 	where
	// 		A: serde::de::SeqAccess<'de>,
	// 	{
	// 		Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(seq))
	// 	}
	// }

	SerAttributeOrAttributeSet::deserialize(deserializer).map(Into::into)
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
enum SerAttribute {
	Str,
	Dex,
	Con,
	Int,
	Wis,
	Cha,
	Free,
}
impl From<SerAttribute> for AttributeSet {
	fn from(value: SerAttribute) -> Self {
		use SerAttribute as A;
		match value {
			A::Str => Attribute::Str.into(),
			A::Dex => Attribute::Dex.into(),
			A::Con => Attribute::Con.into(),
			A::Int => Attribute::Int.into(),
			A::Wis => Attribute::Wis.into(),
			A::Cha => Attribute::Cha.into(),
			A::Free => AttributeSet::all(),
		}
	}
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
enum SerAttributeOrAttributeSet {
	SerAttribute(SerAttribute),
	AttributeSet(AttributeSet),
}
impl From<SerAttributeOrAttributeSet> for AttributeSet {
	fn from(value: SerAttributeOrAttributeSet) -> Self {
		use SerAttributeOrAttributeSet as S;
		match value {
			S::SerAttribute(ser) => ser.into(),
			S::AttributeSet(set) => set,
		}
	}
}

// #[allow(non_camel_case_types)]
// #[derive(Copy, Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
// pub enum Attribute_OLD {
// 	Free = 0xff,
// 	Str = 0x01,
// 	Dex = 0x02,
// 	Con = 0x04,
// 	Int = 0x08,
// 	Wis = 0x10,
// 	Cha = 0x20,
// }
//
// impl Attribute_OLD {
// 	pub fn to_string(attribute: Attribute_OLD) -> Option<String> {
// 		match attribute {
// 			Attribute_OLD::Str => Some("str".to_string()),
// 			Attribute_OLD::Dex => Some("dex".to_string()),
// 			Attribute_OLD::Con => Some("con".to_string()),
// 			Attribute_OLD::Int => Some("int".to_string()),
// 			Attribute_OLD::Wis => Some("wis".to_string()),
// 			Attribute_OLD::Cha => Some("cha".to_string()),
// 			_ => None,
// 		}
// 	}
// }

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
		#[serde(deserialize_with = "deserialize_attribute_set")]
		single_attribute: AttributeSet,
		#[serde(deserialize_with = "deserialize_attribute_set")]
		full: AttributeSet,
		#[serde(deserialize_with = "deserialize_attribute_set")]
		explicit_seq: AttributeSet,
		#[serde(deserialize_with = "deserialize_attribute_set")]
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
