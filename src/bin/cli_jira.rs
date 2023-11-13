use anyhow::{Error, Ok};
use cli_jira::db::JiraDatabase;

fn main() -> Result<(), Error> {
    let json_db = JiraDatabase::new("data/db.json".to_owned());

    let db_state = json_db.read_db()?;

    println!("{:?}", db_state);

    Ok(())
}
