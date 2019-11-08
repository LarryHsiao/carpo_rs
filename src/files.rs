use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

use rusqlite::{params, Connection};

use crate::arch::Source;

/// Source to fetch all files in workspace.
pub struct AllFiles {
    pub root: PathBuf,
}

impl Source<HashSet<String>> for AllFiles {
    fn value(&self) -> Result<HashSet<String>, Box<dyn Error>> {
        let mut result: HashSet<String> = HashSet::new();
        if self.root.is_dir() {
            for entry in fs::read_dir(&self.root)? {
                let file_name = String::from(entry?.path().to_str().unwrap_or_else(|| ""))
                    .replace(&self.root.to_str().unwrap_or_else(|| ""), "");
                result.insert(file_name.clone());
            }
        }
        return Ok(result);
    }
}
