use super::utils::create_test_file;
use biolens_core::commands::validate::fasta::{validator, ValidationOptions};

#[test]
fn test_format_sequence_too_long_fails() {
    let long_sequence = "A".repeat(20);
    let content = format!(">Sequence1\n{}", long_sequence);
    let file = create_test_file(&content);

    let options = ValidationOptions {
        max_sequence_length: Some(10),
        ..Default::default()
    };

    let result = validator::validate_fasta(file.path().to_str().unwrap(), options);
    assert!(
        !result.is_valid,
        "FASTA with sequences longer than maximum length should fail validation"
    );
}
