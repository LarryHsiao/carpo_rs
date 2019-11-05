extern crate confy;

use serde::{Serialize, Deserialize};
use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt)]
struct Cli {
    command: String,
    arg: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, StructOpt)]
struct Config {
    #[structopt(parse(from_os_str))]
    root: PathBuf
}

impl Default for Config {
    fn default() -> Self {
        Self {
            root: PathBuf::from("~/carpo")
        }
    }
}

mod arch;
mod files;

use crate::files::AllFiles;
use crate::arch::Source;

/// Main function of Carpo
fn main() {
    let cfg: Config = confy::load("carpo.config").unwrap();
    #[cfg(debug_assertions)]
    println!("{:#?}", cfg);

    let args = Cli::from_args();
    let command = &args.command;
    match command.as_ref() {
        "setup" => unimplemented!(),
        "list" => {
            AllFiles {
                root: dirs::home_dir().unwrap()
            }.value();
        }
        "serve" => unimplemented!(),
        _ => panic!("Unrecognized command: {}", command)
    }
}