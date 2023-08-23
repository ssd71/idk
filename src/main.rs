use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    // Pattern to look for
    pattern: String,
    path: std::path::PathBuf
}

fn main() {
    let args = Cli::parse();
    println!("{:#?}",args);

    let content = std::fs::read_to_string(args.path).expect("Unable to read file");

    for line in content.lines() {
        println!("{}", line)
    }
}
