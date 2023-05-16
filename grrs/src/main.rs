use std::{path::Path, io, fs};
use clap::Parser;
use anyhow::{Context, Error};
use tempdir::TempDir;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    source: std::path::PathBuf,
    /// The path to the file to read
    destination: std::path::PathBuf,
}


fn main() {
    let args = Cli::parse();

    // pull args into their own variables in one line
    let (source, destination) = (&args.source, &args.destination);

    let result = copy_recursively(source, destination);

    match result {
        Ok(()) => println!("Success!"),
        Err(err) => println!("Error: {}", err),
    }

}


#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}

#[test]
fn copy_a_folder_structure() {
    let temp_dir = TempDir::new("example").unwrap();
    let source = temp_dir.path().join("source");
    let destination = temp_dir.path().join("destination");
    fs::create_dir(&source).unwrap();
    fs::write(source.join("foo.txt"), "lorem ipsum\ndolor sit amet").unwrap();
    fs::create_dir(source.join("bar")).unwrap();
    fs::write(source.join("bar/baz.txt"), "consectetur adipiscing elit").unwrap();

    let result = copy_recursively(&source, &destination);

    assert!(result.is_ok());
    assert!(destination.exists());
    assert!(destination.join("foo.txt").exists());
    assert!(destination.join("bar").exists());
    assert!(destination.join("bar/baz.txt").exists());
}

/// Copy files from source to destination recursively.
fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> io::Result<()> {

    fs::create_dir_all(&destination)?;

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}