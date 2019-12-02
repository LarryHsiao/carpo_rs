use std::collections::HashSet;
use std::error::Error;
use std::path::{Path, PathBuf};

use rusqlite::Connection;
use sciter::dispatch_script_call;
use sciter::make_args;
use sciter::types::HWINDOW;
use sciter::window::Options::DebugMode;
use sciter::Element;

use crate::arch::{Action, Source};
use crate::tags::{AllCFiles, CFileByName, DetachTagAction, FileTags, TagsByName};
use crate::tags::{AttachTagAction, FileSearching, TagByName};
use crate::util::IsImage;

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
        frame
            .set_options(DebugMode(cfg!(debug_assertions)))
            .unwrap();
        frame.event_handler(Events {
            hwnd: frame.get_hwnd(),
            ui: self,
            pwd: self.pwd.clone(),
        });
        frame.load_file("file://html/index.html");
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
        for (key, _) in files {
            let image_uri = {
                if (IsImage {
                    file_name: key.clone(),
                }
                .value()
                .unwrap())
                {
                    let mut image_path = PathBuf::from(self.pwd.clone());
                    image_path.push(key.clone());
                    image_path.into_os_string().into_string().unwrap()
                } else {
                    "".to_string()
                }
            };
            root.call_function("append_file", &make_args!(key, image_uri))
                .unwrap();
        }
    }

    fn search(&self, string: String) {
        let root = Element::from_window(self.hwnd).unwrap();
        root.call_function("clear_files", &[]).unwrap();
        let files = FileSearching {
            keyword: string.as_str(),
            conn: self.ui.conn,
            file_source: self.ui.fs_source,
        }
        .value()
        .unwrap();
        for (key, _) in files {
            root.call_function("append_file", &make_args!(key)).unwrap();
        }
    }

    fn open(&self, file_name: String) {
        open::that(Path::new(&format!("{}/{}", self.pwd, file_name))).unwrap();
    }

    fn load_tags(&self, file_name: String) {
        let root = Element::from_window(self.hwnd).unwrap();
        root.call_function("clear_tags", &[]).unwrap();
        let tags = FileTags {
            conn: self.ui.conn,
            file: &CFileByName {
                conn: self.ui.conn,
                name: file_name.as_str(),
            }
            .value()
            .unwrap(),
        }
        .value()
        .unwrap();
        for (key, _) in tags {
            root.call_function("append_tag", &make_args!(key)).unwrap();
        }
    }

    fn attach_tag(&self, file_name: String, tag_name: String) {
        AttachTagAction {
            conn: self.ui.conn,
            file: &CFileByName {
                name: file_name.as_str(),
                conn: self.ui.conn,
            }
            .value()
            .unwrap(),
            tag: &TagByName {
                conn: self.ui.conn,
                name: tag_name.as_str(),
            }
            .value()
            .unwrap(),
        }
        .fire()
        .unwrap();
        self.load_tags(file_name);
    }

    fn detach_tag(&self, file_name: String, tag_name: String) {
        DetachTagAction {
            conn: self.ui.conn,
            file: &CFileByName {
                name: file_name.as_str(),
                conn: self.ui.conn,
            }
            .value()
            .unwrap(),
            tag: &TagByName {
                conn: self.ui.conn,
                name: tag_name.as_str(),
            }
            .value()
            .unwrap(),
        }
        .fire()
        .unwrap();
        self.load_tags(file_name);
    }

    fn search_tags(&self, tag_name: String) -> String {
        let tags = TagsByName {
            conn: self.ui.conn,
            keyword: tag_name.as_str(),
        }
        .value()
        .unwrap();
        let mut result: Vec<String> = Vec::new();
        for (key, _) in tags {
            result.insert(0, key)
        }
        serde_json::to_string(&result).unwrap()
    }
}

impl sciter::EventHandler for Events<'_> {
    dispatch_script_call! {
        fn load_files();
        fn load_tags(String);
        fn search(String);
        fn open(String);
        fn attach_tag(String, String);
        fn detach_tag(String, String);
        fn search_tags(String);
    }
}
