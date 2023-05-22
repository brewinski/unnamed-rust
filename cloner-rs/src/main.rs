use clap::Parser;
use std::time::{Instant};
mod file_copier;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    #[clap(default_value = "../cloner-rs/")]
    source: std::path::PathBuf,
    /// The path to the file to read
    #[clap(default_value = "./dest/rust/v1")]
    destination: std::path::PathBuf,
}


fn main() {
    let start = Instant::now();
    let args = Cli::parse();

    // pull args into their own variables in one line
    let (source, destination) = (&args.source, &args.destination);

    // for i in 0..100 {
    //     let updated_dest = format!("{}{}", destination.display(), i);
    //     file_copier::copy_recursively(source, updated_dest).unwrap_or_else(|error| {
    //         println!("CopyError:  {}", error);
    //         std::process::exit(1);
    //     });
    // }

    file_copier::copy_recursively(source, destination).unwrap_or_else(|error| {
        println!("CopyError:  {}", error);
        std::process::exit(1);
    });

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
