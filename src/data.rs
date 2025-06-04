// Character struct implementation
// -Alex 2025-06-03

// Todo: Honglos Hüberhaus

use std::collections::HashMap;

pub struct Character {
    level: u8,

    hero_points: u8,

    ancestry: Ancestry,
    heritage: Heritage,
    background: Background,
    class: Class,

    attributes: Attributes,

    saves: Saves,

    damage: u16,
    temp_hp: u16,

    conditions: HashMap<String, u8>,
    damage_types: HashMap<String, DamageScale>,

    skills: Skills,

    languages: Vec<String>,

    proficiencies: Proficiencies,

    feats: Vec<Feat>,

    wealth: Wealth,
}

impl Character {
    fn max_hp(&self) -> u16 {
        return self.ancestry.base_hp as u16 + (self.level as u16)*(self.class.hp as u16 + self.attributes.constitution.bonus as u16);
    }

    fn hurt(&mut self, dmg_amount: u16, dmg_type: String, crit: bool) {
        
        let prescaler: f32;
        
        if let Some(dmg_scale) = self.damage_types.get(&dmg_type) {
            prescaler = match dmg_scale {
                DamageScale::Vulnerable => 2.0,
                DamageScale::Resistant => 0.5,
                DamageScale::Immune => 0.0,
            }
        } else {
            prescaler = 1.0;
        }

        let total_damage = (prescaler * dmg_amount as f32).floor() as u16;

        if total_damage == 0 {return};

        if let Some(dying) = self.conditions.get("Dying") {
            if crit {
                self.conditions.insert(String::from("Dying"), dying + 2);
            } else {
                self.conditions.insert(String::from("Dying"), dying + 1);
            }

            let doomed = self.conditions.get("Doomed").unwrap_or(&0_u8);
            let wounded = self.conditions.get("Wounded").unwrap_or(&0_u8);

            if self.conditions["Dying"] >= (4 - doomed - wounded) {
                self.conditions.insert(String::from("Dying"), 0);
                self.conditions.insert(String::from("Dead"), 1);
                self.conditions.insert(String::from("Doomed"), 0);
            }
        } else {
            if total_damage <= self.temp_hp {
                self.temp_hp -= total_damage;
            } else if total_damage < self.max_hp() + self.damage {
                self.temp_hp = 0;
                self.damage += total_damage;
            } else {
                self.damage = self.max_hp();
                self.conditions.insert(String::from("Dying"), 1);
            }
        }

    }

    fn heal(&mut self, heal_amount: u16){
                
        if heal_amount <= self.damage {
            self.damage -= heal_amount;
        } else {
            self.damage = 0;
        }

        if let Some(_) = self.conditions.get("Dying") {
            self.conditions.insert(String::from("Dying"), 0);

            let wounded = self.conditions.get("Wounded").unwrap_or(&0_u8);

            self.conditions.insert(String::from("Wounded"), wounded + 1);
        }
    }
}

enum DamageScale {
    Vulnerable, 
    Resistant,
    Immune,
}

struct Class {
    id: Identifier,
    key_attribute: Attribute,
    hp: u8,
    saves: Saves,
    skills: Vec<(Skill, Proficiency)>,
    bonus_skills: u8,
    proficiencies: Proficiencies,
    subclasses: Vec<Subclass>,
}

struct Subclass {

}

struct Feat {
    id: Identifier,
    text: String,
}

struct Feature {
    id: Identifier,
    text: String,
}

struct Condition {
    id: String,
    value: i8,
}

struct Skills {
    acrobatics: Proficiency,
    arcana: Proficiency,
    athletics: Proficiency,
    crafting: Proficiency,
    deception: Proficiency,
    diplomacy: Proficiency,
    intimidation: Proficiency,
    medicine: Proficiency,
    nature: Proficiency,
    occultism: Proficiency,
    perception: Proficiency,
    performance: Proficiency,
    religion: Proficiency,
    society: Proficiency,
    stealth: Proficiency,
    survival: Proficiency,
    thievery: Proficiency,
    lore: Vec<(String, Proficiency)>
}

enum Skill {
    Acrobatics,
    Arcana,
    Athletics,
    Crafting,
    Deception,
    Diplomacy,
    Intimidation,
    Medicine,
    Nature,
    Occultism,
    Perception,
    Performance,
    Religion,
    Society,
    Stealth,
    Survival,
    Thievery,
    Lore(String),
}

struct Attributes {
    strength: AttributeValue,
    dexterity: AttributeValue,
    constitution: AttributeValue,
    intelligence: AttributeValue,
    wisdom: AttributeValue,
    charisma: AttributeValue,
}

struct Saves {
    fortitude: Proficiency,
    reflex: Proficiency,
    will: Proficiency,
}

struct Identifier {
    id: String,
    traits: Vec<String>,
}

struct Background {
    id: Identifier,
    fuck_you_morgan: Vec<Attribute>,
    free_attributes: u8,
    skills: Vec<Skill>,
    feats: Vec<Feat>,
    summary: String,
}

struct Heritage {
    id: Identifier,
    features: Vec<Feature>,
}

struct Ancestry {
    id: Identifier,
    size: SizeClass,
    speed: u8,
    base_hp: u8,
    at_boost: Vec<Attribute>,
    at_flaw: Vec<Attribute>,
    base_langs: Vec<String>,
    rec_langs: Vec<String>,
    bonus_langs: u8,
    features: Vec<Feature>,
}

enum SizeClass {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
}

struct AttributeValue {
    bonus: i8,
    partial: bool,
}

impl std::ops::Add<i8> for AttributeValue {
    fn add(self, rhs: i8) -> Self::Output {
        todo!()
    }

    type Output = Self;
}

enum Attribute {
    Free,
    Str,
    Dex,
    Con,
    Int,
    Wis,
    Cha,
}

enum Proficiency {
    Untrained,
    Trained,
    Expert,
    Master,
    Legendary,
}

impl Proficiency {
    fn bonus(&self) -> u8 {
        match self {
            Self::Untrained => 0,
            Self::Trained => 2,
            Self::Expert => 4,
            Self::Master => 6,
            Self::Legendary => 8,
        }
    }
}

struct Proficiencies {
    unarmored: Proficiency,
    light: Proficiency,
    medium: Proficiency,
    heavy: Proficiency,
    unarmed: Proficiency,
    simple: Proficiency,
    martial: Proficiency,
    advanced: Proficiency,
    other: Proficiency,
}

struct Wealth {
    pp: u16,
    gp: u16,
    sp: u16,
    cp: u16,
}