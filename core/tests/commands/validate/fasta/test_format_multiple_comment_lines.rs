use super::utils::create_test_file;
use biolens_core::commands::validate::fasta::{validator, ValidationOptions};

#[test]
fn test_format_multiple_comment_lines() {
    // Create a FASTA file with multiple comment lines
    let content = "; Comment line 1\n; Comment line 2\n>Sequence1\nACGT";
    let file = create_test_file(content);

    let options = ValidationOptions {
        allow_comments: true,
        ..Default::default()
    };

    let result = validator::validate_fasta(file.path().to_str().unwrap(), options);

    // Should be valid with comments allowed
    assert!(
        result.is_valid,
        "FASTA with multiple comment lines should be valid"
    );
    assert_eq!(result.sequence_count, 1, "Should have 1 sequence");
}
