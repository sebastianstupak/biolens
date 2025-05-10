use super::utils::create_test_file;
use biolens_core::commands::validate::fasta::{validator, ValidationOptions};

#[test]
fn test_format_whitespace_in_sequence_fails() {
    let content = ">Sequence1\nACG T\nGCTA";
    let file = create_test_file(content);

    let options = ValidationOptions::default();

    let result = validator::validate_fasta(file.path().to_str().unwrap(), options);
    assert!(
        !result.is_valid,
        "FASTA with whitespace in sequence should fail validation"
    );
}
