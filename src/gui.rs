use std::error::Error;

use sciter::dispatch_script_call;
use sciter::make_args;
use sciter::types::HWINDOW;
use sciter::window::Options::DebugMode;
use sciter::{Element, Value};

use crate::arch::{Action, Source};
use crate::tags::AllCFiles;
use crate::tags::FileSearching;
use rusqlite::Connection;
use std::collections::HashSet;
use std::path::Path;

/// The terminal UI
pub struct UI<'a> {
    pub conn: &'a Connection,
    pub fs_source: &'a dyn Source<HashSet<String>>,
    pub pwd: String,
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
            pwd: self.pwd.clone(),
        });
        frame.load_file(format!("file://{}", path.to_str().unwrap()).as_str());
        frame.run_app();
        Ok(())
    }
}

struct Events<'a> {
    hwnd: HWINDOW,
    ui: &'a UI<'a>,
    pwd: String,
}

impl Events<'_> {
    fn load_files(&self) {
        let root = Element::from_window(self.hwnd).unwrap();
        let files = AllCFiles {
            conn: self.ui.conn,
            fs_source: self.ui.fs_source,
        }
            .value()
            .unwrap();
        for (key, file) in files {
            root.call_function("append_file", &make_args!(key));
        }
    }

    fn search(&self, string: String) {
        let root = Element::from_window(self.hwnd).unwrap();
        root.call_function("clear_files", &[]);
        let files = FileSearching {
            keyword: string.as_str(),
            conn: self.ui.conn,
            file_source: self.ui.fs_source,
        }
            .value()
            .unwrap();
        for (key, file) in files {
            root.call_function("append_file", &make_args!(key));
        }
    }

    fn open(&self, file_name: String) {
        open::that(Path::new(
            &format!("{}/{}", self.pwd, file_name)
        )).unwrap();
    }
}

impl sciter::EventHandler for Events<'_> {
    dispatch_script_call! {
        fn load_files();
        fn search(String);
        fn open(String);
    }
}
