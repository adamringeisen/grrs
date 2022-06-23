#![allow(unused)]
use clap::Parser;
use core::num::ParseIntError;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), ParseIntError> {
    let args = Cli::parse();
    let f = File::open(args.path).expect("Can't Open File");
    let reader = BufReader::new(f);
    let lines = reader.lines();
    let mut count = 0;
    for line in lines {
        let uline = &line.unwrap();
        if uline.contains(&args.pattern) {
            count = count + 1;
            println!("{}", &uline);
        }
    }
    println!("Number of Matches: {}", count);
    Ok(())
}
