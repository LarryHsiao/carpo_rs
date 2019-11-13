extern crate confy;

use std::path::PathBuf;

use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;

use crate::arch::{Action, Source};
use crate::files::AllFiles;
use crate::tags::*;
use std::error::Error;

mod arch;
mod files;
mod tags;

#[derive(StructOpt)]
enum Cli {
    /// Setup the carpo workspace path.
    Setup {
        /// The root path of carpo working on.
        path: String,
    },
    Files {
        #[structopt(subcommand)]
        control: Option<FileControl>,
        tag_name: Option<String>,
    },
    Tags {
        #[structopt(subcommand)]
        control: Option<TagControl>,
        file_name: Option<String>,
    },
    /// Search anything in carpo's workspace.
    Search {
        /// The keyword to search.
        keyword: String,
    },
    Serve {},
}

#[derive(StructOpt, Debug)]
enum FileControl {
    Phantom {/*Place holder*/},
}

#[derive(StructOpt, Debug)]
enum TagControl {
    /// Add a new Tag.
    Add { name: String },
    /// Delete a tag with name.
    Delete { name: String },
    /// Attach a Tag to a exist file.
    Attach {
        /// File name we want the Tag attach to.
        file_name: String,
        /// Tag name we want to do the attach.
        tag_name: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    root: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            root: PathBuf::from(format!(
                "{}{}",
                dirs::home_dir().unwrap().to_str().unwrap(),
                "/carpo_test/"
            )),
        }
    }
}

const CONFIG_NAME: &str = "carpo.config";

/// Main function of Carpo
fn main() {
    let cfg: Config = confy::load(CONFIG_NAME).unwrap();
    #[cfg(debug_assertions)]
    println!("{:#?}", cfg);

    let conn = Connection::open("carpo.db").unwrap();
    TagDb { conn: &conn }.fire().unwrap();

    let args = Cli::from_args();
    match args {
        Cli::Setup { path } => {
            let new_path = PathBuf::from(path);
            if new_path.is_dir() {
                confy::store(CONFIG_NAME, Config { root: new_path }).unwrap()
            }
        }
        Cli::Files { control, tag_name } => match tag_name {
            Some(tag_name) => {
                let results = CFilesByTagName {
                    file_source: &AllFiles { root: cfg.root },
                    conn: &conn,
                    tag_name: tag_name.as_str(),
                };

                for (_, file) in results.value().unwrap() {
                    println!("{}", file.name)
                }
            }
            None => match control {
                Some(control) => unimplemented!(),
                None => {
                    for file in { AllFiles { root: cfg.root }.value().unwrap() } {
                        println!("{}", file)
                    }
                }
            },
        },
        Cli::Tags { control, file_name } => match file_name {
            Some(name) => {
                let file = CFileByName {
                    conn: &conn,
                    name: name.as_str(),
                };
                let results = FileTags {
                    conn: &conn,
                    file: &file.value().unwrap(),
                };
                for (_, tag) in results.value().unwrap() {
                    println!("{}", tag.name);
                }
            }
            None => tag_control(cfg, &conn, control),
        },
        Cli::Search { keyword } => unimplemented!(" @todo #3 Search function.Keyword: {}", keyword),
        Cli::Serve {} => unimplemented!(" @todo #1 http server"),
    }

    fn tag_control(cfg: Config, conn: &Connection, control: Option<TagControl>) {
        match control {
            Some(control) => match control {
                TagControl::Add { name } => {
                    let action = NewTag {
                        conn: &conn,
                        name: name.as_str(),
                    };
                    action.fire().unwrap();
                }
                TagControl::Delete { name } => {
                    let action = TagDeleteByName {
                        conn: &conn,
                        name: name.as_str(),
                    };
                    action.fire().unwrap();
                }
                TagControl::Attach {
                    file_name,
                    tag_name,
                } => {
                    let all_files = AllCFiles {
                        fs_source: &AllFiles { root: cfg.root },
                        conn: &conn,
                    };
                    all_files.value().unwrap(); // to build the file table in db.
                    let tag = TagByName {
                        conn: &conn,
                        name: tag_name.as_str(),
                    };
                    let file = CFileByName {
                        conn: &conn,
                        name: file_name.as_str(),
                    };
                    let attach_action = AttachTagAction {
                        file: &file.value().unwrap(),
                        tag: &tag.value().unwrap(),
                        conn: &conn,
                    };
                    attach_action.fire().unwrap();
                }
            },
            None => {
                for (_name, tag) in { AllTags { conn: &conn }.value().unwrap() } {
                    println!("{}", tag.name)
                }
            }
        }
    }
}
