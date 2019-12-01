use std::fs::File;

use rusqlite::Connection;

use carpo::arch::{Action, Source};
use carpo::files::AllFiles;
use carpo::tags::*;

/// Insert input/output
#[test]
fn insert() {
    let tag_name = "Sample Name";
    let conn = Connection::open_in_memory().unwrap();
    TagDb { conn: &conn }.fire().unwrap();
    let action = NewTag {
        conn: &conn,
        name: tag_name,
    };
    action.fire().unwrap();
    let tags = AllTags { conn: &conn }.value().unwrap();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags.get(tag_name).unwrap().name, tag_name)
}

/// Delete the tag
#[test]
fn delete() {
    let tag_name = "Sample Name";
    let conn = Connection::open_in_memory().unwrap();
    TagDb { conn: &conn }.fire().unwrap();
    let new = NewTag {
        conn: &conn,
        name: tag_name,
    };
    new.fire().unwrap();
    let action = TagDeleteByName {
        conn: &conn,
        name: tag_name,
    };
    action.fire().unwrap();
    let tags = AllTags { conn: &conn }.value().unwrap();
    assert_eq!(tags.len(), 0);
}

/// Normal case of searching by name
#[test]
fn by_name_success() {
    let tag_name = "Sample Name";
    let conn = Connection::open_in_memory().unwrap();
    TagDb { conn: &conn }.fire().unwrap();
    let action = NewTag {
        conn: &conn,
        name: tag_name,
    };
    action.fire().unwrap();
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
    TagDb { conn: &conn }.fire().unwrap();
    let source = TagByName {
        conn: &conn,
        name: tag_name,
    };
    let result = source.value();

    match result {
        Err(_) => assert!(true),
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
    TagDb { conn: &conn }.fire().unwrap();
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
    TagDb { conn: &conn }.fire().unwrap();
    let action = NewTag {
        conn: &conn,
        name: tag_name,
    };
    action.fire().unwrap();
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
        file: files.iter().map(|(_, file)| file).next().unwrap(),
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

#[test]
fn file_by_name() {
    let file_name = "Sample Name";
    let root = tempfile::tempdir().unwrap();
    let file_path = root.path().join(file_name);
    File::create(file_path).unwrap();
    let conn = Connection::open_in_memory().unwrap();
    TagDb { conn: &conn }.fire().unwrap();
    let cfile_source = AllCFiles {
        fs_source: &AllFiles {
            root: root.into_path().clone(),
        },
        conn: &conn,
    };
    cfile_source.value().unwrap();
    let source = CFileByName {
        conn: &conn,
        name: file_name,
    };
    let result = source.value().unwrap();
    assert_eq!(result.name, file_name)
}

#[test]
fn file_by_name_not_found() {
    let conn = Connection::open_in_memory().unwrap();
    let file_name = "Sample Name";
    TagDb { conn: &conn }.fire().unwrap();
    let source = CFileByName {
        conn: &conn,
        name: file_name,
    };
    let result = source.value();

    match result {
        Err(_) => assert!(true),
        _ => assert!(false),
    }
}

/// Attached Tag map returned.
#[test]
fn tags_by_file_id_found() {
    let conn = Connection::open_in_memory().unwrap();
    let root = tempfile::tempdir().unwrap();
    let file_names = ["file1", "file2", "file3"];
    for i in 0..3 {
        let file_path = root.path().join(file_names[i]);
        File::create(file_path).unwrap();
    }
    let tag_name = "Sample Name";
    TagDb { conn: &conn }.fire().unwrap();
    let action = NewTag {
        conn: &conn,
        name: tag_name,
    };
    action.fire().unwrap();
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
    let attached_file = files.iter().map(|(_, file)| file).next().unwrap();
    let attach_action = AttachTagAction {
        file: attached_file,
        tag: &source.value().unwrap(),
        conn: &conn,
    };
    attach_action.fire().unwrap();

    let result = FileTags {
        conn: &conn,
        file: attached_file,
    };
    assert_eq!(result.value().unwrap().len(), 1)
}

/// Return a empty map if no tag attached.
#[test]
fn tags_by_file_id_not_found() {
    let conn = Connection::open_in_memory().unwrap();
    let root = tempfile::tempdir().unwrap();
    let file_names = ["file1", "file2", "file3"];
    for i in 0..3 {
        let file_path = root.path().join(file_names[i]);
        File::create(file_path).unwrap();
    }
    TagDb { conn: &conn }.fire().unwrap();
    let file_source = AllCFiles {
        fs_source: &AllFiles {
            root: root.into_path().clone(),
        },
        conn: &conn,
    };
    let files = &file_source.value().unwrap();
    let target = files.iter().map(|(_, file)| file).next().unwrap();
    let result = FileTags {
        conn: &conn,
        file: target,
    };
    assert_eq!(result.value().unwrap().len(), 0)
}

#[test]
fn file_searching_found() {
    let conn = Connection::open_in_memory().unwrap();
    let root = tempfile::tempdir().unwrap();
    let file_names = ["file1", "file2", "file3"];
    for i in 0..3 {
        let file_path = root.path().join(file_names[i]);
        File::create(file_path).unwrap();
    }
    let tag_name = "Sample Name";
    TagDb { conn: &conn }.fire().unwrap();
    let action = NewTag {
        conn: &conn,
        name: tag_name,
    };
    action.fire().unwrap();
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
    let attached_file = files.iter().map(|(_, file)| file).next().unwrap();
    let attach_action = AttachTagAction {
        file: attached_file,
        tag: &source.value().unwrap(),
        conn: &conn,
    };
    attach_action.fire().unwrap();

    let result_r = FileSearching {
        keyword: tag_name,
        conn: &conn,
        file_source: &AllFiles {
            root: root_path.clone(),
        },
    };
    let result = result_r.value().unwrap();
    assert_eq!(result.len(), 1)
}

/// Insert input/output
#[test]
fn search_tags() {
    let tag_name = "Sample Name";
    let conn = Connection::open_in_memory().unwrap();
    TagDb { conn: &conn }.fire().unwrap();
    for i in 0..3 {
        let action = NewTag {
            conn: &conn,
            name: &format!("Sample Name{}", i),
        };
        action.fire().unwrap();
    }
    let tags = TagsByName {
        conn: &conn,
        keyword: "Sample",
    }
    .value()
    .unwrap();
    assert_eq!(tags.len(), 3);
}
