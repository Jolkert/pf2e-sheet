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
			| Stat::Performance => Attribute::Cha
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
			ron::from_str::<TestAttributeContainer>(ron).expect("oops!"),
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
        
        let serialize= TestAttributeContainer {
			single_attribute: Attribute::Str.into(),
			full: AttributeSet::all(),
			explicit_seq: Attribute::Con | Attribute::Int,
			questionable: Attribute::Dex.into(),
		};
	    
	    assert_eq!(
	        delete_whitespace(&ron::to_string(&serialize).expect("oops!")),
	        delete_whitespace(expected)
	    );
	}

	fn delete_whitespace(string: &str) -> String {
	    string.split_whitespace().fold(String::new(), |mut acc, curr| {
	        acc.push_str(&curr);
	        acc
	    })
	}
}
