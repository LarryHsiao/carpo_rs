use rusqlite::Connection;

use carpo::arch::{Action, Source};
use carpo::files::AllFiles;
use carpo::tags::{AllTags, NewTag, TagDb, TagDeleteByName};

#[test]
fn insert() {
    let tag_name = "Sample Name";
    let conn = Connection::open_in_memory().unwrap();
    TagDb { conn: &conn }.fire();
    let action = NewTag {
        conn: &conn,
        name: tag_name,
    };
    action.fire();
    let tags = AllTags { conn: &conn }.value().unwrap();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags.get(tag_name).unwrap().name, tag_name)
}

#[test]
fn delete() {
    let tag_name = "Sample Name";
    let conn = Connection::open_in_memory().unwrap();
    TagDb { conn: &conn }.fire();
    let new = NewTag {
        conn: &conn,
        name: tag_name,
    };
    new.fire();
    let action = TagDeleteByName {
        conn: &conn,
        name: tag_name,
    };
    action.fire();
    let tags = AllTags { conn: &conn }.value().unwrap();
    assert_eq!(tags.len(), 0);
}
