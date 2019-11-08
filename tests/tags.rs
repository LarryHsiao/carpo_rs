use std::fs::File;
use std::io::{self, Write};
use std::ops::Index;

use rusqlite::Connection;

use carpo::arch::{Action, Source};
use carpo::files::AllFiles;
use carpo::tags::{
    AllCFiles, AllTags, AttachTagAction, CFilesByTagName, NewTag, TagByName, TagDb, TagDeleteByName,
};

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
fn by_name_success() {
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
fn by_name_not_exist() {
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

#[test]
fn all_files_in_workspace() {
    let conn = Connection::open_in_memory().unwrap();
    let root = tempfile::tempdir().unwrap();
    for i in 0..3 {
        let file_path = root.path().join(format!("temp{}", i.to_string()));
        File::create(file_path).unwrap();
    }
    TagDb { conn: &conn }.fire();
    let file_source = AllCFiles {
        fs_source: &AllFiles {
            root: root.into_path(),
        },
        conn: &conn,
    };
    assert_eq!(file_source.value().unwrap().len(), 3)
}

#[test]
fn files_by_tag_name() {
    let conn = Connection::open_in_memory().unwrap();
    let root = tempfile::tempdir().unwrap();
    let file_names = ["file1", "file2", "file3"];
    for i in 0..3 {
        let file_path = root.path().join(file_names[i]);
        File::create(file_path).unwrap();
    }
    let tag_name = "Sample Name";
    TagDb { conn: &conn }.fire();
    let action = NewTag {
        conn: &conn,
        name: tag_name,
    };
    action.fire();
    let root_path = root.into_path();
    let source = &TagByName {
        conn: &conn,
        name: tag_name,
    };
    let file_source = AllCFiles {
        fs_source: &AllFiles {
            root: root_path.clone(),
        },
        conn: &conn,
    };
    let files = &file_source.value().unwrap();
    let attach_action = AttachTagAction {
        file: files.iter().map(|(key, file)| file).next().unwrap(),
        tag: &source.value().unwrap(),
        conn: &conn,
    };
    attach_action.fire().unwrap();

    let result = CFilesByTagName {
        file_source: &AllFiles {
            root: root_path.clone(),
        },
        conn: &conn,
        tag_name: tag_name,
    };
    assert_eq!(result.value().unwrap().len(), 1);
}
