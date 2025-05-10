use super::utils::create_test_file;
use biolens_core::commands::validate::fasta::{
    options::SequenceType, validator, ValidationOptions,
};

#[test]
fn test_format_unicode_characters_fail() {
    // Create sequences with Unicode/non-ASCII characters
    let content = ">Sequence1\nACGT❤️TACG\n>Sequence2\nα-Helix";
    let file = create_test_file(content);

    let options = ValidationOptions {
        sequence_type: SequenceType::Nucleotide,
        ..Default::default()
    };

    let result = validator::validate_fasta(file.path().to_str().unwrap(), options);

    // Debug output
    for issue in &result.issues {
        println!("Issue: {:?} - {}", issue.level, issue.message);
    }

    // The sequences contain invalid characters, so validation should fail
    assert!(
        !result.is_valid,
        "FASTA with Unicode characters should fail validation"
    );

    // Check specifically for invalid character messages
    let has_invalid_char_error = result.issues.iter().any(|issue| {
        issue.message.contains("Invalid characters") || issue.message.contains("invalid characters")
    });

    assert!(
        has_invalid_char_error,
        "Validation should report invalid characters"
    );
}
