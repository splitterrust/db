pub mod schema;
pub mod models;
pub mod json;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

use self::models::{Spell, NewSpell, Schools, NewSchools};

pub fn write_new_schools<'a>(
    conn:          &PgConnection,
    fire:          Option<&'a i32>,
    protection:    Option<&'a i32>,
    ligth:         Option<&'a i32>,
    detection:     Option<&'a i32>,
    strengthening: Option<&'a i32>,
    metamorphism:  Option<&'a i32>,
    bann:          Option<&'a i32>,
    illusion:      Option<&'a i32>,
    movement:      Option<&'a i32>,
    wind:          Option<&'a i32>,
    heal:          Option<&'a i32>,
    death:         Option<&'a i32>,
    fate:          Option<&'a i32>,
    nature:        Option<&'a i32>,
    control:       Option<&'a i32>,
    fight:         Option<&'a i32>,
    water:         Option<&'a i32>,
    shadow:        Option<&'a i32>,
    earth:         Option<&'a i32>
) -> Schools {
    use schema::schools;

    let new_schools = NewSchools {
        fire, protection, ligth, detection, strengthening,
        metamorphism, bann, illusion, movement, wind, heal,
        death, fate, nature, control, fight, water, shadow,
        earth
    };

    diesel::insert_into(schools::table)
        .values(&new_schools)
        .get_result(conn)
        .expect("Error saving new schools")
}

pub fn write_new_spell<'a>(
    conn:           &PgConnection,
    name:           &'a str,
    spell_duration: &'a str,
    options:        &'a Vec<String>,
    reach:          &'a str,
    difficulty:     &'a str,
    typus:          &'a str,
    enforced:       &'a str,
    effect:         &'a str,
    cast_duration:  &'a str,
    cost:           &'a str,
    schools_id:     &'a i32
) -> Spell {
    use schema::spells;

    let new_spell = NewSpell {
        name, spell_duration, options, reach, difficulty, typus,
        enforced, effect, cast_duration, cost, schools_id
    };

    diesel::insert_into(spells::table)
        .values(&new_spell)
        .get_result(conn)
        .expect("Error saving new spell")
}

pub fn write_schools(schools: &NewSchools) -> Schools {
    use schema::schools;

    let conn = establish_connection();

    diesel::insert_into(schools::table)
        .values(schools)
        .get_result(&conn)
        .expect("Error saving new schools")
}

pub fn write_spell(spell: &NewSpell) -> Spell {
    use schema::spells;

    let conn = establish_connection();

    diesel::insert_into(spells::table)
        .values(spell)
        .get_result(&conn)
        .expect("Error saving new spell")
}

pub fn get_spell_by_name(_name: &str) -> Option<Spell> {
    use self::schema::spells::dsl::*;
    let conn = establish_connection();
    let results = spells.filter(name.eq(_name))
          .limit(1)
          .load::<Spell>(&conn)
          .expect("Error finding spell with name");
    match results.len() {
        1 => Some(results[0].clone()),
        _ => None
    }
}

pub fn get_schools_by_id(_id: i32) -> Option<Schools> {
    use self::schema::schools::dsl::*;
    let conn = establish_connection();
    let results = schools.filter(id.eq(_id))
        .limit(1)
        .load::<Schools>(&conn)
        .expect("Error finding schools with id");
    match results.len() {
        1 => Some(results[0].clone()),
        _ => None
    }
}

pub fn read_spells() {
    use self::schema::spells::dsl::*;
    let conn = establish_connection();
    let results = spells
        .limit(5)
        .load::<Spell>(&conn)
        .expect("Error loading spells");

    println!("Displaying {} spells", results.len());
    for spell in results {
        println!("==========================================================");
        println!("{}", spell.name);
        println!("----------");
        println!("{}\n", spell.effect);
    }
}
