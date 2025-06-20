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

#[cfg(test)]
mod tests {
	use super::*;

	#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
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
			ron::from_str::<TestAttributeContainer>(ron)
				.expect("Could not deserialize RON string"),
			expected
		)
	}

	#[test]
	fn attribute_ser_test() {
		let expected = r#"
        (
            single_attribute: Str,
            full: Free,
            explicit_seq: [Con, Int],
            questionable: Dex
        )
	    "#;

		let serialize = TestAttributeContainer {
			single_attribute: Attribute::Str.into(),
			full: AttributeSet::all(),
			explicit_seq: Attribute::Con | Attribute::Int,
			questionable: Attribute::Dex.into(),
		};

		assert_eq!(
			delete_whitespace(
				&ron::to_string(&serialize).expect("Could not serialize testing container")
			),
			delete_whitespace(expected)
		);
	}

	fn delete_whitespace(string: &str) -> String {
		string
			.split_whitespace()
			.fold(String::new(), |mut acc, curr| {
				acc.push_str(&curr);
				acc
			})
	}
}
