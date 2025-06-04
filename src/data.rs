// Character struct implementation
// -Alex 2025-06-03

// Todo: Honglos Hüberhaus

pub struct Character {
    level: u8,

    hero_points: u8,

    ancestry: Ancestry,
    heritage: Heritage,
    background: Background,
    class: Class,

    attributes: Attributes,

    saves: Saves,

    max_hp: u16,
    damage: u16,
    temp_hp: u16,

    conditions: Vec<Condition>,
    resistances: Vec<String>,
    immunities: Vec<String>,
    vulnerabilities: Vec<String>,

    skills: Skills,

    languages: Vec<String>,

    proficiencies: Proficiencies,

    feats: Vec<Feat>,

    wealth: Wealth,
}

struct Class {
    id: Identifier,
    key_attribute: Attribute,
    base_hp: u8,
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
    str: AttributeValue,
    dex: AttributeValue,
    con: AttributeValue,
    int: AttributeValue,
    wis: AttributeValue,
    cha: AttributeValue,
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