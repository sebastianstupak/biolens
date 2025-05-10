use biolens_core::commands::validate::fasta::{validator, ValidationOptions};

#[test]
fn test_format_non_existent_file_fails() {
    // Try to validate a file that doesn't exist
    let options = ValidationOptions::default();
    let result = validator::validate_fasta("non_existent_file.fasta", options);

    // Should fail validation
    assert!(!result.is_valid, "Non-existent file should fail validation");

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
