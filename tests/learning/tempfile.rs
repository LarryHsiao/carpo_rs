use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use tempfile::TempDir;

/// Check the input/output
#[test]
fn read_write_temp_file() {
    let mut tmpfile: File = tempfile::tempfile().unwrap();
    write!(tmpfile, "Hello World!").unwrap();
    tmpfile.seek(SeekFrom::Start(0)).unwrap();
    let mut buf = String::new();
    tmpfile.read_to_string(&mut buf).unwrap();
    assert_eq!("Hello World!", buf);
}

/// Test file read/write in temp dir.
#[test]
fn temp_directory() {
    let mut dir = &tempfile::tempdir().unwrap();
    let mut tmpfile = tempfile::tempfile_in(dir).unwrap();
    assert!(dir.path().exists());
    write!(tmpfile, "Hello World!").unwrap();
    tmpfile.seek(SeekFrom::Start(0)).unwrap();
    let mut buf = String::new();
    tmpfile.read_to_string(&mut buf).unwrap();
    assert_eq!("Hello World!", buf);
}
