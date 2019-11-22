extern crate splitterrust_db;
extern crate diesel;

use diesel::{QueryDsl, RunQueryDsl};

use splitterrust_db::schema::schools;
use splitterrust_db::establish_connection;
use splitterrust_db::models::spell::Spell;
use splitterrust_db::models::schools::Schools;
use splitterrust_db::models::spell_schools::SpellSchools;

fn main() {
    use splitterrust_db::schema::spells::dsl::*;
    let connection = establish_connection();

    let join = spells.left_join(schools::table);
    let all_data = join.load::<(Spell, Option<Schools>)>(&connection).unwrap();

    for d in all_data {
        println!("{:?}", SpellSchools::from_left_join(d).to_json_pretty());
        break;
    }
}
