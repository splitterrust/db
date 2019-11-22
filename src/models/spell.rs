use serde::{Serialize, Deserialize};
use serde_json::json;
use crate::schema::spells;

#[derive(Queryable, Debug, Clone, Serialize, Deserialize)]
pub struct Spell {
    #[serde(skip)]
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
    #[serde(skip)]
    pub schools_id: i32,
}

impl Spell {
    pub fn to_string(self) -> String {
        let obj = json!(self);
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