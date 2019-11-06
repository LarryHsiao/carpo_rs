use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::arch::Source;
use std::error::Error;

pub struct AllFiles {
    pub root: PathBuf
}

pub struct File {
    pub name: String
}

impl Source<Result<HashMap<String, File>, Box<dyn Error>>> for AllFiles {
    fn value(&self) -> Result<HashMap<String, File>, Box<dyn Error>> {
        let mut result: HashMap<String, File> = HashMap::new();
        if self.root.is_dir() {
            for entry in fs::read_dir(&self.root)? {
                let file_name = String::from(entry?.path()
                    .to_str()
                    .unwrap_or_else(|| "")
                ).replace(
                    &self.root.to_str().unwrap_or_else(|| ""),
                    "",
                );
                result.insert(file_name.clone(), File {
                    name: String::from(file_name)
                });
            }
        }
        return Ok(result);
    }
}