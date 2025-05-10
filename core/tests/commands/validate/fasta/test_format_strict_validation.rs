use super::utils::create_test_file;
use biolens_core::commands::validate::fasta::{
    options::SequenceType, validator, ValidationOptions,
};

#[test]
fn test_format_strict_validation() {
    // Create content with several minor issues that wouldn't fail with default options
    let content = ">Sequence with spaces\nacgt\n; A comment line\n\n>Another Sequence\nACGT N";
    let file = create_test_file(content);

    // Configure strict validation options (all checks enabled)
    let options = ValidationOptions {
        sequence_type: SequenceType::Nucleotide,
        check_line_length: true,
        max_line_length: 80,
        min_sequence_length: 1,
        max_sequence_length: Some(1000),
        allow_lowercase: false,        // Require uppercase
        ignore_asterisk_at_end: false, // Don't ignore asterisks
        standardize_sequences: false,  // Don't standardize
        validate_ncbi_headers: true,   // Validate NCBI format
        max_header_length: Some(50),   // Limit header length
        check_extension: true,         // Check extensions
        allow_empty_lines: false,      // Disallow empty lines
        allow_comments: false,         // Disallow comments
    };

    let result = validator::validate_fasta(file.path().to_str().unwrap(), options);

    // With strict validation, we should have multiple issues
    assert!(
        !result.issues.is_empty(),
        "Strict validation should find issues"
    );

    // Debug output
    for issue in &result.issues {
        println!("Issue: {:?} - {}", issue.level, issue.message);
    }

    // With these settings, the file should fail validation
    assert!(!result.is_valid, "FASTA with strict validation should fail");
}
