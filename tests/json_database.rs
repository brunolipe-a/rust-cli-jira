mod common;

use cli_jira::db::{Database, JSONFileDatabase};
use cli_jira::models::{DBState, Epic, RecordStatus, Story};

use std::collections::HashMap;
use std::io::Write;

#[test]
fn read_db_should_fail_with_invalid_path() {
    let db = JSONFileDatabase {
        file_path: "INVALID_PATH".to_owned(),
    };
    assert_eq!(db.read_db().is_err(), true);
}

#[test]
fn read_db_should_fail_with_invalid_json() {
    let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

    let file_contents = r#"{ "last_item_id": 0 epics: {} stories {} }"#;
    write!(tmpfile, "{}", file_contents).unwrap();

    let db = JSONFileDatabase {
        file_path: tmpfile
            .path()
            .to_str()
            .expect("failed to convert tmpfile path to str")
            .to_string(),
    };

    let result = db.read_db();

    assert_eq!(result.is_err(), true);
}

#[test]
fn read_db_should_parse_json_file() {
    let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

    let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
    write!(tmpfile, "{}", file_contents).unwrap();

    let db = JSONFileDatabase {
        file_path: tmpfile
            .path()
            .to_str()
            .expect("failed to convert tmpfile path to str")
            .to_string(),
    };

    let result = db.read_db();

    assert_eq!(result.is_ok(), true);
}

#[test]
fn write_db_should_work() {
    let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

    let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
    write!(tmpfile, "{}", file_contents).unwrap();

    let db = JSONFileDatabase {
        file_path: tmpfile
            .path()
            .to_str()
            .expect("failed to convert tmpfile path to str")
            .to_string(),
    };

    let story = Story {
        name: "epic 1".to_owned(),
        description: "epic 1".to_owned(),
        status: RecordStatus::Open,
    };
    let epic = Epic {
        name: "epic 1".to_owned(),
        description: "epic 1".to_owned(),
        status: RecordStatus::Open,
        stories: vec![2],
    };

    let mut stories = HashMap::new();
    stories.insert(2, story);

    let mut epics = HashMap::new();
    epics.insert(1, epic);

    let state = DBState {
        last_item_id: 2,
        epics,
        stories,
    };

    let write_result = db.write_db(&state);
    let read_result = db.read_db().unwrap();

    assert_eq!(write_result.is_ok(), true);
    assert_eq!(read_result, state);
}
