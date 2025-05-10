use super::utils::create_test_file;
use biolens_core::commands::validate::fasta::{
    options::SequenceType, validator, ValidationOptions,
};

#[test]
fn test_format_lenient_validation() {
    // Create content with several minor issues
    let content = ">Sequence with spaces\nacgt\n; A comment line\n\n>Another Sequence\nACGT N";
    let file = create_test_file(content);

    // Configure lenient validation options (minimal checks)
    let options = ValidationOptions {
        sequence_type: SequenceType::Auto, // Auto-detect type
        check_line_length: false,          // No line length check
        min_sequence_length: 1,            // Allow very short sequences
        max_sequence_length: None,         // No max length
        allow_lowercase: true,             // Allow any case
        ignore_asterisk_at_end: true,      // Ignore trailing asterisks
        standardize_sequences: false,      // Don't standardize
        validate_ncbi_headers: false,      // Don't validate NCBI
        max_header_length: None,           // No header length limit
        check_extension: false,            // Don't check extensions
        allow_empty_lines: true,           // Allow empty lines
        allow_comments: true,              // Allow comments
        ..Default::default()
    };

    let result = validator::validate_fasta(file.path().to_str().unwrap(), options);

    // Debug output
    for issue in &result.issues {
        println!("Issue: {:?} - {}", issue.level, issue.message);
    }

    // With lenient validation, the file should still have issues with spaces in the sequence
    let has_whitespace_error = result.issues.iter().any(|issue| {
        issue.message.contains("Whitespace found") || issue.message.contains("whitespace")
    });

    assert!(
        has_whitespace_error,
        "Even with lenient validation, whitespace in sequences should be detected"
    );

    // With these settings though, whitespace issue will cause it to fail
    assert!(
        !result.is_valid,
        "FASTA with whitespace in sequence should still fail even with lenient validation"
    );
}
