use crate::{stats::*, fluff::*};

pub struct Ancestry {
  pub id: Identifier,
  pub size: SizeClass,
  pub speed: u8,
  pub base_hp: u8,
  pub at_boost: Vec<Attribute>,
  pub at_flaw: Vec<Attribute>,
  pub base_langs: Vec<String>,
  pub rec_langs: Vec<String>,
  pub bonus_langs: u8,
  pub features: Vec<Feature>,
}

enum SizeClass {
  Tiny,
  Small,
  Medium,
  Large,
  Huge,
  Gargantuan,
}