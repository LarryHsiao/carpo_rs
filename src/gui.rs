extern crate azul;

use azul::prelude::*;
use crate::arch::{Action, Source};
use std::error::Error;
use crate::tags::AllCFiles;
use rusqlite::Connection;
use std::collections::HashSet;
use self::azul::dom::NodeType::Div;
use self::azul::widgets::label::Label;
use std::path::PathBuf;
use std::time::Duration;

pub struct GUI<'a> {
    pub fs_source: &'a dyn Source<HashSet<String>>,
    pub conn: &'a Connection,
}

impl Layout for GUI<'_> {
    fn layout(&self, _: LayoutInfo<Self>) -> Dom<Self> {
        let files = AllCFiles {
            fs_source: self.fs_source,
            conn: self.conn,
        }.value().unwrap();
        files.iter().map(|(_, file)| NodeData::label(file.name.clone())).collect::<Dom<Self>>()
            .with_class("list")
    }
}

impl Action for GUI<'_> {
    #[cfg(debug_assertions)]
    fn fire(&self) -> Result<(), Box<dyn Error>> {
        let mut app = App::new(GUI {
            fs_source: self.fs_source,
            conn: self.conn,
        }, AppConfig::default()).unwrap();
        let window = app.create_window(
            WindowCreateOptions::default(),
            css::hot_reload(
                PathBuf::from("src/stylesheet.css"),
                Duration::from_millis(500),
            ).reload_style()?,
        ).unwrap();
        app.run(window).unwrap();
        Ok(())
    }
}