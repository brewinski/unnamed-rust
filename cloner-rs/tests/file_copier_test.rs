use tempdir::TempDir;
use std::fs;

use cloner_rs::copy_recursively;
    
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