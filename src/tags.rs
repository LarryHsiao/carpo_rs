use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::error::Error;

use rusqlite::{params, Connection, NO_PARAMS};

use crate::arch::{Action, Source};

/// Object of tag
pub struct Tag {
    pub name: String,
}

/// Source to build sqlite database connection.
pub struct TagDb<'a> {
    pub conn: &'a Connection,
}

impl Action for TagDb<'_> {
    fn fire(&self) -> Result<(), Box<dyn Error>> {
        // language=SQLite
        &self.conn.execute(
            r#"
            create table if not exists tags(
                id integer primary key autoincrement,
                name text not null unique
            );"#,
            NO_PARAMS,
        )?;
        Ok(())
    }
}

/// Source to build all the Tags in Carpo
pub struct AllTags<'a> {
    pub conn: &'a Connection,
}

impl Source<HashMap<String, Tag>> for AllTags<'_> {
    fn value(&self) -> Result<HashMap<String, Tag, RandomState>, Box<dyn Error>> {
        let mut result: HashMap<String, Tag> = HashMap::new();
        // language=SQLite
        let mut stmt = self.conn.prepare(
            r#"
            SELECT * from tags;
        "#,
        )?;
        let rows = stmt.query_map(NO_PARAMS, |row| Ok(Tag { name: row.get(1)? }))?;

        for row in rows {
            let tag = row?;
            result.insert(tag.name.to_string(), tag);
        }
        Ok(result)
    }
}

/// Source to find or create a Tag that have the given name
pub struct TagByName<'a> {
    pub conn: &'a Connection,
    pub name: &'a str,
}

impl Source<Tag> for TagByName<'_> {
    fn value(&self) -> Result<Tag, Box<dyn Error>> {
        // language=SQLite
        let mut stmt = self.conn.prepare(
            r#"
            SELECT * FROM tags
            WHERE name=(?1);
        "#,
        )?;
        let tags = stmt.query_map(params![self.name], |row| Ok(Tag { name: row.get(1)? }))?;

        for tag in tags {
            return Result::Ok(tag?);
        }
        return Err(format!("No tag found, name={}", self.name))?;
    }
}

/// Action to create a Tag.
pub struct NewTag<'a> {
    pub conn: &'a Connection,
    pub name: &'a str,
}

impl Action for NewTag<'_> {
    fn fire(&self) -> Result<(), Box<dyn Error>> {
        // language=SQLite
        self.conn.execute(
            r#"
            INSERT INTO tags(name)
            values (?1);
        "#,
            &[&self.name],
        )?;
        Ok(())
    }
}

/// A Action to delete a
pub struct TagDeleteByName<'a> {
    pub conn: &'a Connection,
    pub name: &'a str,
}

impl Action for TagDeleteByName<'_> {
    fn fire(&self) -> Result<(), Box<dyn (Error)>> {
        // language=SQLite
        self.conn.execute(
            r#"
            DELETE FROM tags WHERE name=(?1);
        "#,
            &[&self.name],
        )?;
        Ok(())
    }
}
