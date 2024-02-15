use std::{
    fs::File,
    path::PathBuf,
    process::{self},
};

use clap::{Args, Parser, Subcommand};

use crate::nsif::NSIF;

mod nsif;

fn main() {
    let opts = Opts::parse();
    match opts.command {
        Command::Info(InfoArgs { input_file }) => match File::open(input_file) {
            Ok(file) => {
                if let Ok(nsif) = NSIF::parse(&file) {
                    println!("{nsif}");
                } else {
                    eprintln!("Failed to parse given file");
                    process::exit(1);
                }
            }
            Err(_) => {
                eprintln!("Given file path could not be accessed");
                process::exit(1);
            }
        },
    }
}

#[derive(Debug, Parser)]
#[clap(
    author = "Simon Schwarz",
    version = "0.0.1",
    about = "Nimage - A tool for parsing NSIF files"
)]
pub struct Opts {
    /// The command to run
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Display information about the NSIF file
    Info(InfoArgs),
}

#[derive(Debug, Args)]
pub struct InfoArgs {
    /// The path to the nsif file to be parsed
    pub input_file: PathBuf,
}
