use clap::{crate_version, Args, Parser, Subcommand, ValueEnum};
use nimage::nsif::{export::export_to_jpeg, NSIF};
use std::{fs::File, path::PathBuf, process};
use strum::Display;

fn main() {
    let opts = Opts::parse();
    match opts.command {
        Command::Info(InfoArgs {
            input_file,
            print_all_flag,
            print_header_flag,
            print_image_segment_flag,
        }) => match File::open(input_file) {
            Ok(file) => {
                if let Ok(nsif) = NSIF::parse(&file) {
                    if print_image_segment_flag {
                        for (i, image_segment) in nsif.image_segments.into_iter().enumerate() {
                            println!("Image Segment {}:", i + 1);
                            println!("{image_segment}");
                        }
                    } else if print_header_flag {
                        println!("{}", nsif.file_header);
                    } else if print_all_flag {
                        // conflicts annotation, default_value_t annotation & else-if order needed to give us the
                        // inteded effect. Probably need to simplify
                        println!("{nsif}");
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
        Command::Export(ExportArgs {
            input_file,
            output_file,
            segment_type,
            segment_position,
        }) => match File::open(input_file) {
            Ok(file) => {
                if segment_position < 1 {
                    eprintln!("Segment position must be at least 1");
                    process::exit(1);
                }
                if segment_type != SegmentTypeArg::Image {
                    eprintln!("Given Segment type is not implemented yet");
                    process::exit(1);
                }
                if let Ok(nsif) = NSIF::parse(&file) {
                    if let Some(image_segment) = nsif.image_segments.get(segment_position - 1) {
                        if let Err(e) = export_to_jpeg(image_segment, output_file) {
                            eprintln!("Failed to export image segment to file");
                            eprintln!("{e}");
                            process::exit(1);
                        }
                    } else {
                        eprintln!("No image segment detected at this position");
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
    version = crate_version!(),
    about = "NImage - A tool for parsing NSIF files"
)]
pub struct Opts {
    /// The command to run
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Display information about a given NSIF file
    Info(InfoArgs),
    /// Export segments of a given NSIF file as separate files
    Export(ExportArgs),
}

#[derive(Debug, Args)]
pub struct InfoArgs {
    /// The path to the nsif file to be parsed
    pub input_file: PathBuf,
    /// Print all metadata
    #[arg(
        long = "all",
        conflicts_with_all = vec!["print_image_segment_flag", "print_header_flag"],
        action,
        default_value_t = true
    )]
    pub print_all_flag: bool,
    /// Print only file header metadata
    #[arg(long = "header")]
    pub print_header_flag: bool,
    /// Print only image segment metadata
    #[arg(long = "image", conflicts_with_all = vec!["print_all_flag", "print_header_flag"], action)]
    pub print_image_segment_flag: bool,
}

#[derive(Debug, Args)]
pub struct ExportArgs {
    /// The path to the nsif file to be parsed
    pub input_file: PathBuf,
    /// The path of the file to be exported
    pub output_file: PathBuf,
    /// The segment type to be exported
    #[arg(short = 't', long, default_value_t)]
    pub segment_type: SegmentTypeArg,
    /// The position of the segment to be exported
    #[arg(short = 'p', long, default_value = "1")]
    pub segment_position: usize,
}

#[derive(Debug, Clone, Default, ValueEnum, Display, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum SegmentTypeArg {
    #[default]
    Image,
    Graphic,
    Text,
}
