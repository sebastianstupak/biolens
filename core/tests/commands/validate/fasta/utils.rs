use std::fs::File;
use std::io::Write;
use tempfile::NamedTempFile;

pub fn _create_test_file(content: &str) -> NamedTempFile {
    let file = NamedTempFile::new().unwrap();
    let mut file_handle = File::create(file.path()).unwrap();
    file_handle.write_all(content.as_bytes()).unwrap();
    file
}
