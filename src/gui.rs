extern crate azul;

use self::azul::dom::NodeType::Div;
use self::azul::widgets::label::Label;
use crate::arch::{Action, Source};
use crate::tags::AllCFiles;
use crate::tags::CFile;
use azul::prelude::*;
use rusqlite::Connection;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::path::PathBuf;
use std::time::Duration;

struct FileList<'a> {
    pub files: &'a HashMap<String, CFile>,
}

impl Layout for FileList<'_> {
    fn layout(&self, _: LayoutInfo<Self>) -> Dom<Self> {
        self.files
            .iter()
            .map(|(_, file)| NodeData::label(file.name.clone()))
            .collect::<Dom<Self>>()
            .with_class("list")
    }
}

pub struct GUI<'a> {
    pub files: &'a HashMap<String, CFile>,
}

impl Action for GUI<'_> {
    #[cfg(debug_assertions)]
    fn fire(&self) -> Result<(), Box<dyn Error>> {
        let mut app = App::new(FileList { files: self.files }, AppConfig::default()).unwrap();
        let window = app
            .create_hot_reload_window(
                WindowCreateOptions::default(),
                css::hot_reload(
                    PathBuf::from("src/stylesheet.css"),
                    Duration::from_millis(500),
                ),
            )
            .unwrap();
        app.run(window).unwrap();
        Ok(())
    }
}
