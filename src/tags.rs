use crate::arch::Source;
use std::collections::HashMap;
use std::error::Error;
use std::collections::hash_map::RandomState;

/// Source to build all the Tags in Carpo
pub struct AllTags {}

/// Object of tag
pub struct Tag {
    pub name: String
}

impl Source<Result<HashMap<String, Tag>, Box<dyn Error>>> for AllTags {
    fn value(&self) -> Result<HashMap<String, Tag, RandomState>, Box<dyn Error>> {
        unimplemented!(" @todo #2 Source for all tags.")
    }
}