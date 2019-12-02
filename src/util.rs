use std::error::Error;

use crate::arch::Source;

pub struct IsImage {
    pub file_name: String,
}

impl Source<bool> for IsImage {
    fn value(&self) -> Result<bool, Box<dyn Error>> {
        let parts: Vec<&str> = self.file_name.split('.').collect();

        Ok(match parts.last() {
            Some(v) => match *v {
                "png" => true,
                "jpg" => true,
                &_ => false,
            },
            None => false,
        })
    }
}
