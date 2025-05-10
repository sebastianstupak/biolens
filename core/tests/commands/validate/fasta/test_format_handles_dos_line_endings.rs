use biolens_core::commands::validate::fasta::{validator, ValidationOptions};
use std::fs;
use std::io::Write;

#[test]
fn test_format_handles_dos_line_endings() {
    // Create content with CRLF (Windows/DOS) line endings
    let content = ">Sequence1\r\nACGT\r\n>Sequence2\r\nGGGG";

    // We need to use fs directly to ensure CRLF is preserved
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("dos_endings.fasta");
    let mut file = fs::File::create(&file_path).unwrap();
    write!(file, "{}", content).unwrap();

    let options = ValidationOptions::default();
    let result = validator::validate_fasta(file_path.to_str().unwrap(), options);

    // Files with CRLF line endings should be valid
    assert!(
        result.is_valid,
        "FASTA file with DOS/Windows line endings should be valid"
    );
    assert_eq!(result.sequence_count, 2, "Should detect both sequences");
}
