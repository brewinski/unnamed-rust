use std::io;
use std::path::Path;
use std::fs;

pub fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> io::Result<()> {

    // convert source to a path 
    let source = source.as_ref();
    // check path is a directory that exists
    let copy_directory_exists = source.is_dir() && source.exists();

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
