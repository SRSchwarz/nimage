use std::{
    fs::File,
    path::PathBuf,
    process::{self},
};

use clap::{Args, Parser, Subcommand};
use nimage::nsif::{export::export_to_jpeg, NSIF};

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
        Command::Export(ExportArgs {
            input_file,
            output_file,
        }) => match File::open(input_file) {
            Ok(file) => {
                if let Ok(nsif) = NSIF::parse(&file) {
                    let image_segment = nsif.image_segments.get(0).unwrap();
                    if let Err(_) = export_to_jpeg(image_segment, output_file) {
                        eprintln!("Failed to export image segment to file");
                        process::exit(1);
                    }
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
    /// Export segments as separate files
    Export(ExportArgs),
}

#[derive(Debug, Args)]
pub struct InfoArgs {
    /// The path to the nsif file to be parsed
    pub input_file: PathBuf,
}

#[derive(Debug, Args)]
pub struct ExportArgs {
    /// The path to the nsif file to be parsed
    pub input_file: PathBuf,
    /// The path of the file to be exported
    pub output_file: PathBuf,
}
