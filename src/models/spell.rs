/*
This is the Spell data, which represents the data in the spell table. No joins
were made here.
*/
use std::fmt;
use crate::schema::spells;
use diesel::pg::PgConnection;
use diesel::RunQueryDsl;
use serde::{
    Deserialize,
    Serialize,
};
use serde_json::json;

#[derive(Queryable, Debug, Clone, Serialize, Deserialize)]
pub struct Spell {
    pub id:                 i32,
    pub name:               String,
    pub cast_duration:      String,
    pub options:            Vec<String>,
    pub range:              String,
    pub difficulty:         String,
    pub typus:              String,
    pub enforced:           String,
    pub effect:             String,
    pub duration_of_effect: String,
    pub cost:               String,
    pub schools_id:         i32,
}

impl fmt::Display for Spell {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let obj = json!(self);
        write!(f, "{}", serde_json::to_string_pretty(&obj).unwrap())
    }
}

#[derive(Insertable, Debug)]
#[table_name = "spells"]
pub struct NewSpell<'a> {
    pub name:               &'a str,
    pub cast_duration:      &'a str,
    pub options:            &'a Vec<String>,
    pub range:              &'a str,
    pub difficulty:         &'a str,
    pub typus:              &'a str,
    pub enforced:           &'a str,
    pub effect:             &'a str,
    pub duration_of_effect: &'a str,
    pub cost:               &'a str,
    pub schools_id:         &'a i32,
}

pub fn write_new_spell(
    conn: &PgConnection,
    new_spell: NewSpell
) -> Spell {

    diesel::insert_into(spells::table)
        .values(&new_spell)
        .get_result(conn)
        .expect("Error saving new spell")
}

pub fn write_spell(spell: &NewSpell, conn: &PgConnection) -> Spell {
    diesel::insert_into(spells::table)
        .values(spell)
        .get_result(conn)
        .expect("Error saving new spell")
}
