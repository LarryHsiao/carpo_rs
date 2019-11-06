use crate::arch::{Source, Action};
use std::collections::HashMap;
use std::error::Error;
use std::collections::hash_map::RandomState;

/// Object of tag
pub struct Tag {
    pub name: String
}

/// Source to build all the Tags in Carpo
pub struct AllTags {}

impl Source<HashMap<String, Tag>> for AllTags {
    fn value(&self) -> Result<HashMap<String, Tag, RandomState>, Box<dyn Error>> {
        unimplemented!(" @todo #2 Source for all tags.")
    }
}

/// Source to find or create a Tag that have the given name
pub struct TagByName {
    name: String
}

impl Source<Tag> for TagByName {
    fn value(&self) -> Result<Tag, Box<dyn Error>> {
        unimplemented!(" @todo #5 Create/Build Tag by name.")
    }
}

/// Action to create a Tag.
pub struct NewTag {
    name: String
}

impl Action for NewTag {
    fn fire(&self) -> Result<(), Box<dyn Error>> {
        unimplemented!(" @todo #4 new Tag")
    }
}

/// A Action to delete a
pub struct TagDeleteByName {
    name: String
}

impl Action for TagDeleteByName {
    fn fire(&self) -> Result<(), Box<dyn (Error)>> {
        unimplemented!(" @todo $5 Delete Tag by name")
    }
}