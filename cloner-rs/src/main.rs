use clap::Parser;
use std::time::{Instant};
mod file_copier;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    source: std::path::PathBuf,
    /// The path to the file to read
    destination: std::path::PathBuf,
}


fn main() {
    let start = Instant::now();
    let args = Cli::parse();

    // pull args into their own variables in one line
    let (source, destination) = (&args.source, &args.destination);

    file_copier::copy_recursively(source, destination).unwrap_or_else(|error| {
        println!("CopyError:  {}", error);
        std::process::exit(1);
    });

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
