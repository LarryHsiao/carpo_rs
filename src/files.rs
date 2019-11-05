use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs::read_dir;
use std::fs;
use crate::arch::Source;

pub struct AllFiles {
    pub root: PathBuf
}

pub struct File {}

impl Source<HashMap<String, File>> for AllFiles {
    fn value(&self) -> HashMap<String, File> {
        let result = HashMap::new();
        if self.root.is_dir() {
            for entry in fs::read_dir(&self.root).unwrap() {
                let entry2 = entry.unwrap();
                println!("{}", entry2.path().to_str().unwrap())
            }
        }
        return result;
    }
}