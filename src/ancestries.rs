use std::collections::HashMap;
use crate::{stats::*, fluff::*};
use serde::{Deserialize, Serialize};

#[derive (Debug, Deserialize, Serialize)]
pub struct Ancestry {
  pub id: Identifier,
  pub size: Vec<SizeClass>,
  pub speed: HashMap<String, u8>,
  pub base_hp: u8,
  pub at_boost: Vec<Attribute>,
  pub at_flaw: Vec<Attribute>,
  pub base_langs: Vec<String>,
  pub rec_langs: Vec<String>,
  pub bonus_langs: u8,
  pub features: Vec<Feature>,
}

#[derive (Debug, Deserialize, Serialize)]
pub enum SizeClass {
  Tiny,
  Small,
  Medium,
  Large,
  Huge,
  Gargantuan,
}

#[test]
fn test_serde() {
  let file_str = std::fs::read_to_string("ancestries/nagaji.ron").expect("FUCK (1)");
  let _: Ancestry = ron::from_str(&file_str).expect("FUCK (2)");
}