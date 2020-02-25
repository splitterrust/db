extern crate splitterrust_db;

use self::splitterrust_db::establish_connection;
use self::splitterrust_db::json::utils::write_json_spells_to_db;

fn main() {
    let conn = establish_connection().get().map_err(|_e| {}).unwrap();

    write_json_spells_to_db(&conn);
}
