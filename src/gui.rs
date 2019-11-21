use crate::arch::Action;
use std::error::Error;

/// The terminal UI
pub struct UI {}

impl Action for UI {
    fn fire(&self) -> Result<(), Box<dyn Error>> {
        let mut frame = sciter::Window::new();
        let mut path = std::env::current_dir()?;
        path.push("html");
        path.push("index.html");
        frame.load_file(format!("file://{}", path.to_str().unwrap()).as_str());
        frame.run_app();
        Ok(())
    }
}
