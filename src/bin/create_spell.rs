extern crate diesel;
extern crate splitterrust_db;

use self::splitterrust_db::*;

fn main() {
    let connection = establish_connection();

    let schools = write_new_schools(
        &connection,
        Option::None,
        Option::None,
        Option::None,
        Option::None,
        Option::None,
        Option::None,
        Option::None,
        Option::None,
        Option::None,
        Option::None,
        Option::None,
        Option::None,
        Option::None,
        Option::None,
        Option::None,
        Option::None,
        Option::None,
        Option::None,
        Option::None,
    );
    println!("Wrote school: {:?}", schools);

    let spell = write_new_spell(
        &connection,
        "name",
        "options",
        &vec![String::from("one"), String::from("two")],
        "reach",
        "difficulty",
        "typus",
        "enforced",
        "effect",
        "cast_duration",
        "cost",
        &schools.id,
    );
    println!("Wrote spell: {:?}", spell);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
