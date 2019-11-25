use std::error::Error;

use sciter::dispatch_script_call;
use sciter::make_args;
use sciter::types::HWINDOW;
use sciter::window::Options::DebugMode;
use sciter::{Element, Value};

use crate::arch::{Action, Source};
use crate::tags::AllCFiles;
use rusqlite::Connection;
use std::collections::HashSet;

/// The terminal UI
pub struct UI<'a> {
    pub conn: &'a Connection,
    pub fs_source: &'a dyn Source<HashSet<String>>,
}

impl Action for UI<'_> {
    fn fire(&self) -> Result<(), Box<dyn Error>> {
        let mut frame = sciter::WindowBuilder::main_window()
            .with_size((1280, 720))
            .create();
        frame.set_options(DebugMode(cfg!(debug_assertions)));
        let mut path = std::env::current_dir()?;
        path.push("html");
        path.push("index.html");
        frame.event_handler(Events {
            hwnd: frame.get_hwnd(),
            ui: self,
        });
        frame.load_file(format!("file://{}", path.to_str().unwrap()).as_str());
        frame.run_app();
        Ok(())
    }
}

struct Events<'a> {
    hwnd: HWINDOW,
    ui: &'a UI<'a>,
}

impl Events<'_> {
    fn load_files(&self) {
        let root = Element::from_window(self.hwnd).unwrap();
        root.call_function(
            "load_files",
            &make_args!(
                1 //            AllCFiles {
                  //                conn: self.ui.conn,
                  //                fs_source: self.ui.fs_source,
                  //            }.value().unwrap().len()
            ),
        );
    }
}

impl sciter::EventHandler for Events<'_> {
    dispatch_script_call! {
        fn load_files();
    }
}
