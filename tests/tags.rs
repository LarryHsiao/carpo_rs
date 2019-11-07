use rusqlite::Connection;

use carpo::arch::{Action, Source};
use carpo::files::AllFiles;
use carpo::tags::{AllTags, NewTag, TagByName, TagDb, TagDeleteByName};

/// Insert input/output
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

/// Delete the tag
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

/// Normal case of searching by name
#[test]
fn byName_success() {
    let tag_name = "Sample Name";
    let conn = Connection::open_in_memory().unwrap();
    TagDb { conn: &conn }.fire();
    let action = NewTag {
        conn: &conn,
        name: tag_name,
    };
    action.fire();
    let source = TagByName {
        conn: &conn,
        name: tag_name,
    };
    let tag = source.value().unwrap();
    assert_eq!(tag.name, tag_name)
}

/// Searching by name panic when no tag founded.
#[test]
fn byName_not_exist() {
    let tag_name = "Sample Name";
    let conn = Connection::open_in_memory().unwrap();
    TagDb { conn: &conn }.fire();
    let source = TagByName {
        conn: &conn,
        name: tag_name,
    };
    let result = source.value();

    match result {
        Err(error) => assert!(true),
        _ => assert!(false),
    }
}
