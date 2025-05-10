use biolens_core::commands::validate::fasta::{validator, ValidationOptions};
use std::fs;
use std::io::Write;

#[test]
fn test_format_reads_file_without_newline() {
    // Create a file that doesn't end with a newline
    let content = ">Sequence1\nACGT"; // No trailing newline

    // We need to use fs directly since create_test_file might add a newline
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("no_newline.fasta");
    let mut file = fs::File::create(&file_path).unwrap();
    write!(file, "{}", content).unwrap();
    // No newline at end of file

    let options = ValidationOptions::default();
    let result = validator::validate_fasta(file_path.to_str().unwrap(), options);

    // A file without a trailing newline should still be valid
    assert!(
        result.is_valid,
        "FASTA file without trailing newline should be valid"
    );
}
