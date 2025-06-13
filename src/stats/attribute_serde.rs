use enumset::EnumSet;

use crate::stats::{Attribute, AttributeSet};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename = "Attribute")]
pub enum SerAttribute {
	Str,
	Dex,
	Con,
	Int,
	Wis,
	Cha,
	Free,
}
impl From<SerAttribute> for EnumSet<Attribute> {
	fn from(value: SerAttribute) -> Self {
		use SerAttribute as A;
		match value {
			A::Str => Attribute::Str.into(),
			A::Dex => Attribute::Dex.into(),
			A::Con => Attribute::Con.into(),
			A::Int => Attribute::Int.into(),
			A::Wis => Attribute::Wis.into(),
			A::Cha => Attribute::Cha.into(),
			A::Free => EnumSet::all(),
		}
	}
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum SerAttributeOrAttributeSet {
	SerAttribute(SerAttribute),
	AttributeSet(EnumSet<Attribute>),
}
impl From<SerAttributeOrAttributeSet> for EnumSet<Attribute> {
	fn from(value: SerAttributeOrAttributeSet) -> Self {
		use SerAttributeOrAttributeSet as S;
		match value {
			S::SerAttribute(ser) => ser.into(),
			S::AttributeSet(set) => set,
		}
	}
}

impl serde::Serialize for AttributeSet {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		if self == &Self::all() {
			SerAttribute::Free.serialize(serializer)
		} else if self.len() == 1 {
			self.iter().next().unwrap().serialize(serializer)
		} else {
			self.0.serialize(serializer)
		}
	}
}
impl<'de> serde::Deserialize<'de> for AttributeSet {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		SerAttributeOrAttributeSet::deserialize(deserializer)
			.map(|deser| Self(EnumSet::from(deser)))
	}
}
