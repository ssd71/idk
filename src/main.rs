use clap::Parser;
use std::error::Error;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

#[derive(Parser, Debug)]
struct Cli {
    // Pattern to look for
    pattern: String,
    path: std::path::PathBuf
}

fn main() -> Result<(), Box<dyn Error>>{
    let args = Cli::parse();
    println!("{:#?}",args);

    let file = File::open(args.path)?;
    let file = match File::open(args.path) {
        Ok(f) => f,
        Err(e) => return Err(e.into())
    };

    let mut reader = BufReader::new(file);

    let mut line = String::new();

    loop {
        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }
        println!("{}", line);
    }
    
    Ok(())
}
