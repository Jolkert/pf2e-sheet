#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum DamageType {
	Bludgeoning,
	Piercing,
	Slashing,
	Acid,
	Cold,
	Electricity,
	Fire,
	Sonic,
	Spirit,
	Mental,
	// TODO: is this right? -morgan 2025-06-08
	Poison,
	Bleed,
}
