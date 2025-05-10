use super::utils::create_test_file;
use biolens_core::commands::validate::fasta::{validator, ValidationOptions};

#[test]
fn test_format_empty_file_fails_alternative() {
    let content = "";
    let file = create_test_file(content);
    let options = ValidationOptions::default();
    let validation_result = validator::validate_fasta(file.path().to_str().unwrap(), options);

    assert!(
        !validation_result.is_valid,
        "Empty FASTA file should fail validation"
    );
}
