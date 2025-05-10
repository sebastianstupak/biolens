use super::utils::create_test_file;
use biolens_core::commands::validate::fasta::{validator, ValidationOptions};

#[test]
fn test_format_header_too_long_warning() {
    // Create a very long header
    let long_header = "S".repeat(200);
    let content = format!(">{}\nACGT", long_header);
    let file = create_test_file(&content);

    let options = ValidationOptions {
        max_header_length: Some(100), // Set maximum header length
        ..Default::default()
    };

    let result = validator::validate_fasta(file.path().to_str().unwrap(), options);

    // This should generate a warning, not necessarily fail validation
    let has_length_warning = result.issues.iter().any(|issue| {
        issue.message.contains("header exceeds maximum length")
            || issue.message.contains("Header exceeds maximum length")
    });

    assert!(
        has_length_warning,
        "FASTA with too long headers should generate warnings"
    );
}
