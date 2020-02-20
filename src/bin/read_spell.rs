extern crate diesel;
extern crate splitterrust_db;

use self::diesel::prelude::*;
use self::models::spell::Spell;
use self::splitterrust_db::*;

fn main() {
    use splitterrust_db::schema::spells::dsl::*;

    let conn = establish_connection().get().map_err(|_e| {});

    let results = spells
        .limit(5)
        .load::<Spell>(&conn.unwrap())
        .expect("Error loading spells");

    println!("Displaying {} spells", results.len());
    for spell in results {
        println!("{}", spell.name);
        println!("----------\n");
        println!("{}", spell.effect);
    }
}
