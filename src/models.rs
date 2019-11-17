use serde_json::json;
use super::get_schools_by_id;
use super::schema::{spells, schools};
use super::json::types::Spell as Jspell;
use super::json::types::Schools as Jschools;

#[derive(Queryable, Debug, Clone)]
pub struct Spell {
    pub id: i32,
    pub name: String,
    pub spell_duration: String,
    pub options: Vec<String>,
    pub reach: String,
    pub difficulty: String,
    pub typus: String,
    pub enforced: String,
    pub effect: String,
    pub cast_duration: String,
    pub cost: String,
    pub schools_id: i32,
}

impl Spell {
    fn to_json_struct(self) -> Jspell {
        let schools = get_schools_by_id(self.schools_id).unwrap();
        Jspell {
            spell_duration: self.spell_duration,
            options: self.options,
            reach: self.reach,
            difficulty: self.difficulty,
            typus: self.typus,
            enforced: self.enforced,
            effect: self.effect,
            cast_duration: self.cast_duration,
            cost: self.cost,
            name: self.name,
            schools: schools.to_json_struct(),
        }
    }

    pub fn to_string(self) -> String {
        let obj = json!(self.to_json_struct());
        serde_json::to_string_pretty(&obj).unwrap()
    }
}

#[derive(Insertable, Debug)]
#[table_name="spells"]
pub struct NewSpell <'a> {
    pub name: &'a str,
    pub spell_duration: &'a str,
    pub options: &'a Vec<String>,
    pub reach: &'a str,
    pub difficulty: &'a str,
    pub typus: &'a str,
    pub enforced: &'a str,
    pub effect: &'a str,
    pub cast_duration: &'a str,
    pub cost: &'a str,
    pub schools_id: &'a i32
}

#[derive(Queryable, Debug, Clone)]
pub struct Schools {
    pub id: i32,
    pub fire: Option<i32>,
    pub protection: Option<i32>,
    pub ligth: Option<i32>,
    pub detection: Option<i32>,
    pub strengthening: Option<i32>,
    pub metamorphism: Option<i32>,
    pub bann: Option<i32>,
    pub illusion: Option<i32>,
    pub movement: Option<i32>,
    pub wind: Option<i32>,
    pub heal: Option<i32>,
    pub death: Option<i32>,
    pub fate: Option<i32>,
    pub nature: Option<i32>,
    pub control: Option<i32>,
    pub fight: Option<i32>,
    pub water: Option<i32>,
    pub shadow: Option<i32>,
    pub earth: Option<i32>
}

impl Schools {
    pub fn to_json_struct(self) -> Jschools {
        Jschools {
            fire: self.fire,
            protection: self.protection,
            light: self.ligth,
            detection: self.detection,
            strengthening: self.strengthening,
            metamorphism: self.metamorphism,
            bann: self.bann,
            illusion: self.illusion,
            movement: self.movement,
            wind: self.wind,
            heal: self.heal,
            death: self.detection,
            fate: self.fate,
            nature: self.nature,
            control: self.control,
            fight: self.fight,
            water: self.water,
            shadow: self.shadow,
            earth: self.earth
        }
    }

    pub fn to_string(self) -> String {
        let obj = json!(self.to_json_struct());
        serde_json::to_string_pretty(&obj).unwrap()
    }
}

#[derive(Insertable, Debug)]
#[table_name="schools"]
pub struct NewSchools <'a> {
    pub fire: Option<&'a i32>,
    pub protection: Option<&'a i32>,
    pub ligth: Option<&'a i32>,
    pub detection: Option<&'a i32>,
    pub strengthening: Option<&'a i32>,
    pub metamorphism: Option<&'a i32>,
    pub bann: Option<&'a i32>,
    pub illusion: Option<&'a i32>,
    pub movement: Option<&'a i32>,
    pub wind: Option<&'a i32>,
    pub heal: Option<&'a i32>,
    pub death: Option<&'a i32>,
    pub fate: Option<&'a i32>,
    pub nature: Option<&'a i32>,
    pub control: Option<&'a i32>,
    pub fight: Option<&'a i32>,
    pub water: Option<&'a i32>,
    pub shadow: Option<&'a i32>,
    pub earth: Option<&'a i32>
}
