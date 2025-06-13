use enumset::EnumSet;

use crate::stats::{Attribute, AttributeSet};

// Attribute <op> Attribute -> AttributeSet
impl std::ops::BitOr for Attribute {
	type Output = AttributeSet;

	fn bitor(self, rhs: Self) -> Self::Output {
		AttributeSet(EnumSet::from(self) | rhs)
	}
}
impl std::ops::BitAnd for Attribute {
	type Output = AttributeSet;

	fn bitand(self, rhs: Self) -> Self::Output {
		AttributeSet(EnumSet::from(self) & rhs)
	}
}
impl std::ops::BitXor for Attribute {
	type Output = AttributeSet;

	fn bitxor(self, rhs: Self) -> Self::Output {
		AttributeSet(EnumSet::from(self) ^ rhs)
	}
}

// AttributeSt <op> AttributeSet -> AttributeSet
impl std::ops::BitOr for AttributeSet {
	type Output = Self;

	fn bitor(self, rhs: Self) -> Self::Output {
		Self(self.0 | rhs.0)
	}
}
impl std::ops::BitAnd for AttributeSet {
	type Output = Self;

	fn bitand(self, rhs: Self) -> Self::Output {
		Self(self.0 & rhs.0)
	}
}
impl std::ops::BitXor for AttributeSet {
	type Output = Self;

	fn bitxor(self, rhs: Self) -> Self::Output {
		Self(self.0 ^ rhs.0)
	}
}
impl std::ops::Sub for AttributeSet {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Self(self.0 - rhs.0)
	}
}
impl std::ops::Not for AttributeSet {
	type Output = Self;

	fn not(self) -> Self::Output {
		Self(!self.0)
	}
}

// AttributeSet <op> Attribute -> AttributeSet
impl std::ops::BitOr<Attribute> for AttributeSet {
	type Output = Self;

	fn bitor(self, rhs: Attribute) -> Self::Output {
		Self(self.0 | rhs)
	}
}
impl std::ops::BitAnd<Attribute> for AttributeSet {
	type Output = Self;

	fn bitand(self, rhs: Attribute) -> Self::Output {
		Self(self.0 & rhs)
	}
}
impl std::ops::BitXor<Attribute> for AttributeSet {
	type Output = Self;

	fn bitxor(self, rhs: Attribute) -> Self::Output {
		Self(self.0 ^ rhs)
	}
}
impl std::ops::Sub<Attribute> for AttributeSet {
	type Output = AttributeSet;

	fn sub(self, rhs: Attribute) -> Self::Output {
		Self(self.0 - rhs)
	}
}
