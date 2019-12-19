pub mod json;
pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use self::models::schools::{
    NewSchools,
    Schools,
};
use self::models::spell::{
    NewSpell,
    Spell,
};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn get_spell_by_name(_name: &str) -> Option<(Spell, Schools)> {
    use self::schema::schools;
    use self::schema::spells::dsl::*;

    let conn = establish_connection();

    let results = spells
        .filter(name.eq(_name))
        .limit(1)
        .inner_join(schools::table)
        .load::<(Spell, Schools)>(&conn)
        .expect("Error retrieving spell");

    match results.len() {
        0 => None,
        _ => Some(results[0].clone()),
    }
}

pub fn get_spell_like_name(_name: &str) -> Vec<(Spell, Schools)> {
    use self::schema::schools;
    use self::schema::spells::dsl::*;

    let conn = establish_connection();

    let query = format!("{}", _name);
    spells
        .filter(name.ilike(query))
        .inner_join(schools::table)
        .load::<(Spell, Schools)>(&conn)
        .expect("Error finding spell like")
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
