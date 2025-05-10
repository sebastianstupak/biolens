use super::utils::create_test_file;
use biolens_core::commands::validate::fasta::{validator, ValidationOptions};
#[test]
fn test_format_whitespace_only_fails() {
    let content = "   ";
    let file = create_test_file(content);
    let options = ValidationOptions::default();
    let validation_result = validator::validate_fasta(file.path().to_str().unwrap(), options);

    assert!(
        !validation_result.is_valid,
        "FASTA file with only whitespaces should fail validation"
    );
}
