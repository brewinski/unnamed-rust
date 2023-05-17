use std::io;
use std::path::Path;
use std::fs;
use tempdir::TempDir;

pub fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> io::Result<()> {

    // convert source to a path 
    let source = source.as_ref();
    // check path is a directory that exists
    let copy_directory_exists = source.is_dir() && source.exists();

    // println!("copy_directory_exists: {}, {}", copy_directory_exists, source.display());

    if !copy_directory_exists {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Copy directory does not exist"));
    }

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

#[test]
pub fn copy_a_folder_structure() {
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
