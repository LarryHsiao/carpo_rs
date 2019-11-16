use std::path::{Path, PathBuf};

use rusqlite::Connection;
use structopt::StructOpt;

use crate::arch::{Action, Source};
use crate::files::AllFiles;
use crate::tags::*;
use crate::gui::GUI;

mod arch;
mod files;
mod tags;
mod gui;

#[derive(StructOpt)]
enum Cli {
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
    GUI {},
}

#[derive(StructOpt, Debug)]
enum FileControl {
    Phantom { /*Place holder*/ },
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

/// Main function of Carpo
fn main() {
    let pwd = std::env::current_dir().unwrap();
    let pwd_string = pwd.clone().into_os_string().into_string().unwrap();
    let args = Cli::from_args();

    let carpo_db = PathBuf::from(format!("{}/carpo.db", pwd_string));
    if !Path::new(&carpo_db).exists() {
        use std::io::{stdin, stdout, Write};
        let mut s = String::new();
        println!("Initialize the carpo here? [y/n]");
        let _ = stdout().flush();
        stdin().read_line(&mut s).unwrap();
        if s.starts_with('n') {
            return;
        }
    }

    let conn_r = Connection::open(format!("{}/carpo.db", pwd_string));
    let conn = conn_r.unwrap();
    TagDb { conn: &conn }.fire().unwrap();

    match args {
        Cli::Files { control, tag_name } => match tag_name {
            Some(tag_name) => {
                let results = CFilesByTagName {
                    file_source: &AllFiles { root: pwd.clone() },
                    conn: &conn,
                    tag_name: tag_name.as_str(),
                };

                for (_, file) in results.value().unwrap() {
                    println!("{}", file.name)
                }
            }
            None => match control {
                Some(_control) => unimplemented!(),
                None => {
                    for file in { AllFiles { root: pwd.clone() }.value().unwrap() } {
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
            None => tag_control(pwd, &conn, control),
        },
        Cli::Search { keyword } => {
            let source = FileSearching {
                keyword: keyword.as_str(),
                conn: &conn,
                file_source: &AllFiles { root: pwd },
            };
            let results = source.value().unwrap();
            for (_, file) in results {
                println!("{}", file.name);
            }
        }
        Cli::Serve {} => unimplemented!(" @todo #1 http server"),
        Cli::GUI {} => GUI {
            fs_source: &AllFiles { root: pwd },
            conn: &conn
        }.fire().unwrap(),
    }

    fn tag_control(root: PathBuf, conn: &Connection, control: Option<TagControl>) {
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
                        fs_source: &AllFiles { root: root },
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
