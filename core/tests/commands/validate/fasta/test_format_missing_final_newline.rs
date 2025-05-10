use biolens_core::commands::validate::fasta::{validator, ValidationOptions};
use std::fs;
use std::io::Write;

#[test]
fn test_format_missing_final_newline() {
    // Create a file with multiple sequences where the last one doesn't have a newline
    let content = ">Sequence1\nACGT\n>Sequence2\nGGGG"; // No trailing newline

    // We need to use fs directly to ensure no trailing newline
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("no_final_newline.fasta");
    let mut file = fs::File::create(&file_path).unwrap();
    write!(file, "{}", content).unwrap();

    let options = ValidationOptions::default();
    let result = validator::validate_fasta(file_path.to_str().unwrap(), options);

    // Should be valid - missing final newline is acceptable
    assert!(
        result.is_valid,
        "FASTA with missing final newline should be valid"
    );
    assert_eq!(result.sequence_count, 2, "Should have 2 sequences");
}
