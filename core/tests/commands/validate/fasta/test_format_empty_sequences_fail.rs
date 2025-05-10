use super::utils::create_test_file;
use biolens_core::commands::validate::fasta::{validator, ValidationOptions};

#[test]
fn test_format_empty_sequences_fail() {
    // Create a FASTA file with an empty sequence
    let content = ">Sequence1\n>Sequence2\nACGT";
    let file = create_test_file(content);

    let options = ValidationOptions::default();
    let result = validator::validate_fasta(file.path().to_str().unwrap(), options);

    // Should fail - empty sequences are not valid
    assert!(
        !result.is_valid,
        "FASTA with empty sequence should fail validation"
    );

    // Check for specific error about empty sequence
    let has_empty_seq_error = result.issues.iter().any(|issue| {
        issue.message.contains("Empty sequence") || issue.message.contains("empty sequence")
    });

    assert!(
        has_empty_seq_error,
        "Should report error about empty sequence"
    );
}
