use super::utils::create_test_file;
use biolens_core::commands::validate::fasta::{validator, ValidationOptions};

#[test]
fn test_format_empty_header_fails() {
    let content = ">\nACGT"; // Empty header
    let file = create_test_file(content);

    let options = ValidationOptions::default();

    let result = validator::validate_fasta(file.path().to_str().unwrap(), options);
    assert!(
        !result.is_valid,
        "FASTA with empty header should fail validation"
    );
}
