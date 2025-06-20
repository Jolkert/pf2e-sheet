use crate::data::Meta;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Armor {
	pub meta: Meta,
	pub price: u16,
	pub bulk: u16,
	pub ac_bonus: u8,
	pub dex_cap: i8,
	pub check_penalty: i8,
	pub speed_penalty: i8,
	pub str_req: u8,
	pub category: ArmorCategory,
	pub group: ArmorGroup,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ArmorCategory {
	Unarmored,
	Light,
	Medium,
	Heavy,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ArmorGroup {
	Chain,
	Cloth,
	Composite,
	Leather,
	Plate,
	Skeletal,
	Wood,
}
