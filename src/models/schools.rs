/*
This is the Schools data, which represents the data in the schools table.
*/
use crate::establish_connection;
use crate::schema::schools;
use diesel::pg::PgConnection;
use diesel::RunQueryDsl;
use serde::{
    Deserialize,
    Serialize,
};
use serde_json::json;

#[derive(Queryable, Debug, Clone, Serialize, Deserialize)]
pub struct Schools {
    pub id:            i32,
    pub fire:          Option<i32>,
    pub protection:    Option<i32>,
    pub light:         Option<i32>,
    pub detection:     Option<i32>,
    pub strengthening: Option<i32>,
    pub metamorphism:  Option<i32>,
    pub bann:          Option<i32>,
    pub illusion:      Option<i32>,
    pub movement:      Option<i32>,
    pub wind:          Option<i32>,
    pub heal:          Option<i32>,
    pub death:         Option<i32>,
    pub fate:          Option<i32>,
    pub nature:        Option<i32>,
    pub control:       Option<i32>,
    pub fight:         Option<i32>,
    pub water:         Option<i32>,
    pub shadow:        Option<i32>,
    pub earth:         Option<i32>,
}

impl Schools {
    pub fn to_string(self) -> String {
        let obj = json!(self);
        serde_json::to_string_pretty(&obj).unwrap()
    }
}

#[derive(Insertable, Debug)]
#[table_name = "schools"]
pub struct NewSchools<'a> {
    pub fire:          Option<&'a i32>,
    pub protection:    Option<&'a i32>,
    pub light:         Option<&'a i32>,
    pub detection:     Option<&'a i32>,
    pub strengthening: Option<&'a i32>,
    pub metamorphism:  Option<&'a i32>,
    pub bann:          Option<&'a i32>,
    pub illusion:      Option<&'a i32>,
    pub movement:      Option<&'a i32>,
    pub wind:          Option<&'a i32>,
    pub heal:          Option<&'a i32>,
    pub death:         Option<&'a i32>,
    pub fate:          Option<&'a i32>,
    pub nature:        Option<&'a i32>,
    pub control:       Option<&'a i32>,
    pub fight:         Option<&'a i32>,
    pub water:         Option<&'a i32>,
    pub shadow:        Option<&'a i32>,
    pub earth:         Option<&'a i32>,
}

pub fn write_new_schools<'a>(
    conn: &PgConnection,
    fire: Option<&'a i32>,
    protection: Option<&'a i32>,
    light: Option<&'a i32>,
    detection: Option<&'a i32>,
    strengthening: Option<&'a i32>,
    metamorphism: Option<&'a i32>,
    bann: Option<&'a i32>,
    illusion: Option<&'a i32>,
    movement: Option<&'a i32>,
    wind: Option<&'a i32>,
    heal: Option<&'a i32>,
    death: Option<&'a i32>,
    fate: Option<&'a i32>,
    nature: Option<&'a i32>,
    control: Option<&'a i32>,
    fight: Option<&'a i32>,
    water: Option<&'a i32>,
    shadow: Option<&'a i32>,
    earth: Option<&'a i32>,
) -> Schools {
    let new_schools = NewSchools {
        fire,
        protection,
        light,
        detection,
        strengthening,
        metamorphism,
        bann,
        illusion,
        movement,
        wind,
        heal,
        death,
        fate,
        nature,
        control,
        fight,
        water,
        shadow,
        earth,
    };

    diesel::insert_into(schools::table)
        .values(&new_schools)
        .get_result(conn)
        .expect("Error saving new schools")
}

pub fn write_schools(schools: &NewSchools) -> Schools {
    let conn = establish_connection();

    diesel::insert_into(schools::table)
        .values(schools)
        .get_result(&conn)
        .expect("Error saving new schools")
}
