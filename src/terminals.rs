use crate::arch::Action;
use std::error::Error;

/// The terminal UI
pub struct UI {}

impl Action for UI {
    fn fire(&self) -> Result<(), Box<dyn Error>> {
        unimplemented!(" @todo #10 terminal uis")
    }
}
