use anyhow::{Error, Ok};
use db::Database;

mod db;
mod models;

fn main() -> Result<(), Error> {
    let json_db = db::JSONFileDatabase {
        file_path: "data/db.json".to_owned(),
    };

    let db_state = json_db.read_db()?;

    println!("{:?}", db_state);

    Ok(())
}
