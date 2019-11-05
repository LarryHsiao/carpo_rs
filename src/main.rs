extern crate confy;

mod arch;
mod files;

use serde::{Serialize, Deserialize};
use structopt::StructOpt;
use std::path::PathBuf;

use crate::files::AllFiles;
use crate::arch::Source;

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
            root: PathBuf::from(
                format!(
                    "{}{}", dirs::home_dir().unwrap().to_str().unwrap(), "/carpo_test/"
                )
            )
        }
    }
}

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
                root: cfg.root
            }.value();
        }
        "serve" => unimplemented!(),
        _ => panic!("Unrecognized command: {}", command)
    }
}