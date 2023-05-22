use std::io;
use std::path::Path;
use std::fs;
use std::thread;
use std::io::{prelude::*};

pub fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> io::Result<()> {
    let source = source.as_ref();
    let copy_directory_exists = source.is_dir() && source.exists();

    if !copy_directory_exists {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Copy directory does not exist"));
    }

    fs::create_dir_all(&destination)?;

    let mut copy_threads: Vec<thread::JoinHandle<()>> = Vec::new();

    for entry in fs::read_dir(&source)? {
        let entry = entry?;
        let src_path = entry.path();
        let filetype = entry.file_type()?;
        let dest_path = destination.as_ref().join(entry.file_name());

        if filetype.is_dir() {
            copy_threads.push(thread::spawn(move || {
                copy_recursively(&src_path, &dest_path).unwrap_or_else(|error| {
                    println!("CopyError:  {}", error);
                    std::process::exit(1);
                });
            }));
        }
    }

    // Wait for all the threads to finish
    for thread in copy_threads {
        thread.join().unwrap();
    }

    for entry in fs::read_dir(&source)? {
        let entry = entry?;
        let src_path = entry.path();
        let filetype = entry.file_type()?;
        let dest_path = destination.as_ref().join(entry.file_name());

        if !filetype.is_dir() {
            copy_file(&src_path, &dest_path);
        }
    }

    Ok(())
}

fn copy_file(source_path: impl AsRef<Path>, dest_path: impl AsRef<Path>) {
    let mut file = fs::File::open(&source_path).unwrap_or_else(|error| {
        println!("CopyError:  {}", error);
        std::process::exit(1);
    });
    let mut content = String::new();
    file.read_to_string(&mut content);

    let replaced_content = content.replace("m", "<Z>");
    let mut dst = fs::File::create(&dest_path).unwrap_or_else(|open_error| {
        println!("CopyError:  {}", open_error);
        std::process::exit(1);
    });
    dst.write(replaced_content.as_bytes()).unwrap_or_else(|error| {
        println!("CopyError:  {}", error);
        std::process::exit(1);
    });
}
