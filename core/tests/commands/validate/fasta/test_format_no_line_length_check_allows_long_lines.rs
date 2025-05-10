use super::utils::create_test_file;
use biolens_core::commands::validate::fasta::{validator, ValidationOptions};

#[test]
fn test_format_no_line_length_check_allows_long_lines() {
    // Create a sequence line longer than the default max (80 characters)
    let long_line = "A".repeat(100);
    let content = format!(">Sequence1\n{}", long_line);
    let file = create_test_file(&content);

    let options = ValidationOptions {
        check_line_length: false,
        ..Default::default()
    };

    let result = validator::validate_fasta(file.path().to_str().unwrap(), options);
    let has_length_warning = result
        .issues
        .iter()
        .any(|issue| issue.message.contains("exceeds maximum length"));

    assert!(
        !has_length_warning,
        "FASTA with line length check disabled should not warn about long lines"
    );
    assert!(
        result.is_valid,
        "FASTA with line length check disabled should be valid"
    );
}
