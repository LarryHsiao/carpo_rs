use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::{PathBuf, MAIN_SEPARATOR};

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
                let mut file_name = String::from(entry?.path().to_str().unwrap_or_else(|| ""))
                    .replace(&self.root.to_str().unwrap_or_else(|| ""), "");
                if file_name.starts_with(MAIN_SEPARATOR) {
                    file_name.replace_range(0..1, "")
                }
                if file_name.eq(&"carpo.db".to_string()) {
                    continue;
                }
                result.insert(file_name.clone());
            }
        }
        return Ok(result);
    }
}
