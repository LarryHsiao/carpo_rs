extern crate confy;

use serde::{Serialize, Deserialize};
use structopt::StructOpt;
use std::path::PathBuf;
use std::process;

#[derive(StructOpt)]
struct Cli {
    command: String,
    arg: String,
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

fn main() {
    let cfg: Config = confy::load("carpo.config").unwrap();
    #[cfg(debug_assertions)]
    println!("{:#?}", cfg);

    let args = Cli::from_args();
    let command = &args.command;
    match command.as_ref() {
        "setup" => eprintln!("setup: in construction"),
        "list" => eprintln!("list: function still in construction"),
        _ => panic!("Unrecognized command: {}", command)
    }
}