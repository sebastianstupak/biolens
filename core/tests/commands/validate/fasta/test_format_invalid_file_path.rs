use biolens_core::commands::validate::fasta::{validator, ValidationOptions};

#[test]
fn test_format_invalid_file_path() {
    // Try to validate a file with invalid characters in the path
    let options = ValidationOptions::default();

    // This path contains characters that shouldn't be valid on any filesystem
    let result = validator::validate_fasta("this/path/has/invalid/\0/null/characters", options);

    // Should fail validation
    assert!(
        !result.is_valid,
        "File with invalid path should fail validation"
    );

    // Should have a specific error about file opening
    let has_file_error = result.issues.iter().any(|issue| {
        issue.message.contains("Failed to open file")
            || issue.message.contains("file") && issue.message.contains("open")
    });

    assert!(
        has_file_error,
        "Should report error about failing to open file"
    );
}
