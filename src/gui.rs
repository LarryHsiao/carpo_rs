use crate::arch::Action;
use std::error::Error;
use sciter::types::SCITER_RT_OPTIONS::SCITER_SET_DEBUG_MODE;
use sciter::window::Options::{DebugMode, AlphaWindow, TransparentWindow};

/// The terminal UI
pub struct UI {}

impl Action for UI {
    fn fire(&self) -> Result<(), Box<dyn Error>> {
        let mut frame = sciter::Window::new();
        frame.set_options(DebugMode(cfg!(debug_assertions)));
        let mut path = std::env::current_dir()?;
        path.push("html");
        path.push("index.html");
        frame.load_file(format!("file://{}", path.to_str().unwrap()).as_str());
        frame.run_app();
        Ok(())
    }
}
