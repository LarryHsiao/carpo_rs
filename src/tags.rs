use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};
use std::error::Error;

use rusqlite::{params, Connection, NO_PARAMS};

use crate::arch::{Action, Source};

/// Object of tag
pub struct Tag {
    pub id: i64,
    pub name: String,
}

/// Source to build sqlite database connection.
pub struct TagDb<'a> {
    pub conn: &'a Connection,
}

/// Object to represent a File in workspace.
pub struct CFile {
    pub id: i64,
    pub name: String,
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
        // language=SQLite
        &self.conn.execute(
            r#"
               CREATE TABLE IF NOT EXISTS files(
                    id integer primary key autoincrement,
                    path text not null unique
               );
            "#,
            NO_PARAMS,
        );
        // language=SQLite
        &self.conn.execute(
            r#"
                CREATE TABLE IF NOT EXISTS files_tags(
                    id integer primary key autoincrement,
                    file_id integer not null ,
                    tag_id integer not null ,
                    unique (file_id, tag_id)
                );
            "#,
            NO_PARAMS,
        );
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
        let rows = stmt.query_map(NO_PARAMS, |row| {
            Ok(Tag {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;

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
        let tags = stmt.query_map(params![self.name], |row| {
            Ok(Tag {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;

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

pub struct AllCFiles<'a> {
    pub fs_source: &'a dyn Source<HashSet<String>>,
    pub conn: &'a Connection,
}

impl Source<HashMap<String, CFile>> for AllCFiles<'_> {
    fn value(&self) -> Result<HashMap<String, CFile, RandomState>, Box<dyn Error>> {
        let mut files = self.fs_source.value()?;
        let mut result: HashMap<String, CFile> = HashMap::new();
        let mut stmt = self.conn.prepare(
            // language=SQLite
            r#"
                SELECT * FROM files;
            "#,
        )?;
        let db_file_rows = stmt.query_map(params![], |row| {
            Ok(CFile {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;

        for db_file_row in db_file_rows {
            let c_file = db_file_row?;
            if files.contains(&c_file.name) {
                let key = c_file.name.clone();
                result.insert(key.clone(), c_file);
                files.remove(&key);
            }
        }
        for file in files {
            let mut stmt = self.conn.prepare(
                // language=SQLite
                r#"
                INSERT INTO files(path)
                VALUES ((?1));
                "#,
            )?;
            stmt.execute(params![file])?;
            result.insert(
                file.clone(),
                CFile {
                    id: self.conn.last_insert_rowid(),
                    name: file.clone(),
                },
            );
        }
        Ok(result)
    }
}

/// Source to find Files that have given tag name.
pub struct CFilesByTagName<'a> {
    pub file_source: &'a dyn Source<HashSet<String>>,
    pub conn: &'a Connection,
    pub tag_name: &'a str,
}

impl Source<HashMap<String, CFile>> for CFilesByTagName<'_> {
    fn value(&self) -> Result<HashMap<String, CFile>, Box<dyn Error>> {
        let files: HashSet<String> = self.file_source.value()?;
        let mut result: HashMap<String, CFile> = HashMap::new();
        let mut stmt = self.conn.prepare(
            // language=SQLite
            r#"
                SELECT files.* FROM tags LEFT JOIN files, files_tags
                WHERE tags.id=files_tags.tag_id AND
                      files.id=files_tags.file_id AND
                      tags.name=(?1);
            "#,
        )?;
        let db_files = stmt.query_map(params![self.tag_name], |row| {
            Ok(CFile {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;
        for db_file in db_files {
            let db_file = db_file?;
            if files.contains(&db_file.name) {
                result.insert(db_file.name.clone(), db_file);
            }
            // @todo #6 if the file not exist on file system.
        }
        return Ok(result);
    }
}

/// Action to do the link an Tag to File.
pub struct AttachTagAction<'a> {
    pub file: &'a CFile,
    pub tag: &'a Tag,
    pub conn: &'a Connection,
}

impl Action for AttachTagAction<'_> {
    fn fire(&self) -> Result<(), Box<dyn Error>> {
        self.conn.execute(
            // language=SQLite
            r#"
            INSERT INTO files_tags(file_id, tag_id)
            VALUES ((?1), (?2));
            "#,
            params![self.file.id, self.tag.id],
        )?;
        Ok(())
    }
}
