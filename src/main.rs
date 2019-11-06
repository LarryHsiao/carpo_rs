extern crate confy;

mod arch;
mod files;

use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use std::path::PathBuf;

use crate::files::AllFiles;
use crate::arch::Source;

#[derive(StructOpt)]
struct Cli {
    command: String,
    arg: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
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

const CONFIG_NAME: &str = "carpo.config";

/// Main function of Carpo
fn main() {
    let cfg: Config = confy::load(CONFIG_NAME).unwrap();
    #[cfg(debug_assertions)]
    println!("{:#?}", cfg);

    let args = Cli::from_args();
    let command = &args.command;
    match command.as_ref() {
        "setup" => {
            let new_path = PathBuf::from(args.arg.unwrap());
            if new_path.is_dir() {
                confy::store(
                    CONFIG_NAME,
                    Config {
                        root: new_path
                    },
                ).unwrap()
            }
        }
        "list" => {
            for (_name, file) in {
                AllFiles { root: cfg.root }.value().unwrap()
            } {
                println!("{}", file.name)
            }
        }
        "serve" => unimplemented!(),
        _ => panic!("Unrecognized command: {}", command)
    }
}