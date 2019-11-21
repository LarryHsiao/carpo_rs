use crate::arch::Action;
use std::error::Error;

/// The terminal UI
pub struct UI {}

impl Action for UI {
    fn fire(&self) -> Result<(), Box<dyn Error>> {
        let mut frame = sciter::Window::new();
        frame.load_file("html/index.html");
        frame.run_app();
        Ok(())
    }
}
