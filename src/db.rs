use anyhow::Result;
use std::fs;

use crate::models::{DBState, Epic, RecordStatus, Story};

pub struct JiraDatabase {
    pub database: Box<dyn Database>,
}

impl JiraDatabase {
    pub fn new(file_path: String) -> Self {
        JiraDatabase {
            database: Box::new(JSONFileDatabase { file_path }),
        }
    }

    pub fn read_db(&self) -> Result<DBState> {
        self.database.read_db()
    }

    pub fn create_epic(&self, epic: Epic) -> Result<u32> {
        todo!()
    }

    pub fn create_story(&self, story: Story, epic_id: u32) -> Result<u32> {
        todo!()
    }

    pub fn delete_epic(&self, epic_id: u32) -> Result<()> {
        todo!()
    }

    pub fn delete_story(&self, epic_id: u32, story_id: u32) -> Result<()> {
        todo!()
    }

    pub fn update_epic_status(&self, epic_id: u32, status: RecordStatus) -> Result<()> {
        todo!()
    }

    pub fn update_story_status(&self, story_id: u32, status: RecordStatus) -> Result<()> {
        todo!()
    }
}

pub trait Database {
    fn read_db(&self) -> Result<DBState>;
    fn write_db(&self, db_state: &DBState) -> Result<()>;
}

pub struct JSONFileDatabase {
    pub file_path: String,
}

impl Database for JSONFileDatabase {
    fn read_db(&self) -> Result<DBState> {
        let db_content = fs::read_to_string(&self.file_path)?;
        let db_state: DBState = serde_json::from_str(&db_content)?;
        Ok(db_state)
    }

    fn write_db(&self, db_state: &DBState) -> Result<()> {
        fs::write(&self.file_path, serde_json::to_string(db_state)?)?;

        Ok(())
    }
}
