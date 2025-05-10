use super::utils::create_test_file;
use biolens_core::commands::validate::fasta::{validator, ValidationOptions};

#[test]
fn test_format_long_sequence_lines_warning() {
    // Create a sequence line longer than the default max (80 characters)
    let long_line = "A".repeat(100);
    let content = format!(">Sequence1\n{}", long_line);
    let file = create_test_file(&content);

    let options = ValidationOptions {
        check_line_length: true,
        max_line_length: 80,
        ..Default::default()
    };

    let result = validator::validate_fasta(file.path().to_str().unwrap(), options);
    // This should cause a warning, not an error
    let has_length_warning = result
        .issues
        .iter()
        .any(|issue| issue.message.contains("exceeds maximum length"));

    assert!(
        has_length_warning,
        "FASTA with long lines should generate a warning"
    );
    // File should still be valid since this is just a warning
    assert!(
        result.is_valid,
        "FASTA with long sequence lines should still be valid"
    );
}
