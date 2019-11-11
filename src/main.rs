extern crate confy;

use std::path::PathBuf;

use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;

use crate::arch::{Action, Source};
use crate::files::AllFiles;
use crate::tags::{AllTags, TagDb};

mod arch;
mod files;
mod tags;

#[derive(StructOpt)]
struct Cli {
    command: String,
    arg: Option<String>,
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
    let command = &args.command;
    match command.as_ref() {
        "setup" => {
            let new_path = PathBuf::from(args.arg.unwrap());
            if new_path.is_dir() {
                confy::store(CONFIG_NAME, Config { root: new_path }).unwrap()
            }
        }
        "files" => {
            for file in { AllFiles { root: cfg.root }.value().unwrap() } {
                println!("{}", file)
            }
        }
        "tags" => {
            for (_name, tag) in { AllTags { conn: &conn }.value().unwrap() } {
                println!("{}", tag.name)
            }
        }
        "search" => {
            args.arg.expect("No keyword provided.");
            unimplemented!(" @todo #3 Search function.")
        }
        "serve" => unimplemented!(" @todo #1 http server"),
        _ => panic!("Unrecognized command: {}", command),
    }
}
