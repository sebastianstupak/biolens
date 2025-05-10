use super::utils::create_test_file;
use biolens_core::commands::validate::fasta::{validator, ValidationOptions};

#[test]
fn test_format_sequence_too_short_fails() {
    let content = ">Sequence1\nA";
    let file = create_test_file(content);

    let options = ValidationOptions {
        min_sequence_length: 2,
        ..Default::default()
    };

    let result = validator::validate_fasta(file.path().to_str().unwrap(), options);
    assert!(
        !result.is_valid,
        "FASTA with sequences shorter than minimum length should fail validation"
    );
}
