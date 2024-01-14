use nsif::NSIF;
use std::{env, fs::File, process};

mod nsif;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(path) = args.get(1) {
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
        eprintln!("No path was given");
        process::exit(1)
    }
}
