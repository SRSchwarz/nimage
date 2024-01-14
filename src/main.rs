use nsif::NSIF;
use std::{
    env::{self},
    fs::File,
    process,
};

mod nsif;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(command) = args.get(1) {
        if command != "info" {
            eprintln!("Only the 'info' command is supported at this time");
            process::exit(1)
        }
    }
    if let Some(path) = args.get(2) {
        match File::open(path) {
            Ok(file) => match NSIF::parse(&file) {
                Ok(nsif) => {
                    println!("{}", nsif);
                }
                Err(_) => {
                    eprintln!("There were problems parsing the given nsif file");
                    process::exit(1)
                }
            },
            Err(_) => {
                eprintln!("File not found");
                process::exit(1)
            }
        }
    } else {
        eprintln!("No path or command was given");
        process::exit(1)
    }
}
