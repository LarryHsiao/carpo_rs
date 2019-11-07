use carpo::files::AllFiles;
use carpo::arch::{Source, Action};

use rusqlite::Connection;
use carpo::tags::{AllTags, TagDb, NewTag, TagDeleteByName};

#[test]
fn insert() {
    let tag_name = "Sample Name";
    let conn = Connection::open_in_memory().unwrap();
    TagDb { conn: &conn }.fire();
    NewTag { conn: &conn, name: tag_name }.fire();
    let tags = AllTags { conn: &conn }.value().unwrap();
    assert_eq!(tags.len(), 1);
    assert_eq!(
        tags.get(tag_name).unwrap().name,
        tag_name
    )
}

#[test]
fn delete() {
    let tag_name = "Sample Name";
    let conn = Connection::open_in_memory().unwrap();
    TagDb { conn: &conn }.fire();
    NewTag { conn: &conn, name: tag_name }.fire();
    TagDeleteByName { conn: &conn, name: tag_name }.fire();
    let tags = AllTags { conn: &conn }.value().unwrap();
    assert_eq!(tags.len(), 0);
}

