/*
This is the Schools data, which represents the data in the schools table.
*/
use crate::schema::schools;
use diesel::pg::PgConnection;
use diesel::RunQueryDsl;
use serde::{
    Deserialize,
    Serialize,
};
use serde_json::json;
use std::fmt;

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

impl fmt::Display for Schools {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let obj = json!(self);
        write!(f, "{}", serde_json::to_string_pretty(&obj).unwrap())
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

pub fn write_schools(schools: &NewSchools, conn: &PgConnection) -> Schools {
    diesel::insert_into(schools::table)
        .values(schools)
        .get_result(conn)
        .expect("Error saving new schools")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_schools_to_string_1() {
        let school = Schools {
            id:            1,
            fire:          Some(1),
            protection:    Some(1),
            light:         Some(1),
            detection:     Some(1),
            strengthening: Some(1),
            metamorphism:  Some(1),
            bann:          Some(1),
            illusion:      Some(1),
            movement:      Some(1),
            wind:          Some(1),
            heal:          Some(1),
            death:         Some(1),
            fate:          Some(1),
            nature:        Some(1),
            control:       Some(1),
            fight:         Some(1),
            water:         Some(1),
            shadow:        Some(1),
            earth:         Some(1),
        };
        let obj = json!(school);
        let s_string = serde_json::to_string_pretty(&obj).unwrap();
        assert_eq!(s_string, school.to_string());
    }

    #[test]
    fn test_schools_to_string_2() {
        let school = Schools {
            id:            1,
            fire:          Some(1),
            protection:    Some(1),
            light:         Some(1),
            detection:     Some(1),
            strengthening: Some(1),
            metamorphism:  Some(1),
            bann:          Some(1),
            illusion:      Some(1),
            movement:      Some(1),
            wind:          Some(1),
            heal:          Some(1),
            death:         Some(1),
            fate:          Some(1),
            nature:        Some(1),
            control:       Some(1),
            fight:         Some(1),
            water:         Some(1),
            shadow:        Some(1),
            earth:         Some(1),
        };
        let s = String::from(
            "{\n  \"bann\": 1,\n  \"control\": 1,\n  \"death\": 1,\n  \"detection\": 1,\n  \
             \"earth\": 1,\n  \"fate\": 1,\n  \"fight\": 1,\n  \"fire\": 1,\n  \"heal\": 1,\n  \
             \"id\": 1,\n  \"illusion\": 1,\n  \"light\": 1,\n  \"metamorphism\": 1,\n  \
             \"movement\": 1,\n  \"nature\": 1,\n  \"protection\": 1,\n  \"shadow\": 1,\n  \
             \"strengthening\": 1,\n  \"water\": 1,\n  \"wind\": 1\n}",
        );
        assert_eq!(s, school.to_string());
    }
}
