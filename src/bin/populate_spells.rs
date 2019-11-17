extern crate splitterrust_db;

use self::splitterrust_db::json::utils::write_json_spells_to_db;

fn main() {
    write_json_spells_to_db();
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
