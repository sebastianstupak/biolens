use biolens_core::commands::validate::fasta_validator;
use std::fs::File;
use std::io::Write;
use tempfile::NamedTempFile;

fn create_test_file(content: &str) -> NamedTempFile {
    let file = NamedTempFile::new().unwrap();
    let mut file_handle = File::create(file.path()).unwrap();
    file_handle.write_all(content.as_bytes()).unwrap();
    file
}

#[test]
fn test_valid_fasta() {
    let content = ">Sequence1\nACGT\n";
    let file = create_test_file(content);

    let result = fasta_validator::validate_fasta(file.path().to_str().unwrap());
    assert!(result, "Valid FASTA file should pass validation");
}
