use clap::Parser;
mod file_copier;
use std::time::{Duration, Instant};

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

// #[test]
// fn find_a_match() {
//     let mut result = Vec::new();
//     find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
//     assert_eq!(result, b"lorem ipsum\n");
// }

// fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
//     for line in content.lines() {
//         if line.contains(pattern) {
//             writeln!(writer, "{}", line);
//         }
//     }
// }
