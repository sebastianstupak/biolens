use super::utils::create_test_file;
use biolens_core::commands::validate::fasta::{validator, ValidationOptions};

#[test]
fn test_format_ncbi_invalid_headers() {
    // Invalid NCBI format (missing required parts)
    let content = ">gb||\nACGT"; // Empty accession is invalid
    let file = create_test_file(content);

    let options = ValidationOptions {
        validate_ncbi_headers: true,
        ..Default::default()
    };

    let result = validator::validate_fasta(file.path().to_str().unwrap(), options);

    // This should generate a warning, not necessarily fail validation
    let has_ncbi_warning = result.issues.iter().any(|issue| {
        issue.message.contains("NCBI format")
            || issue.message.contains("format standards")
            || issue.message.contains("conform to NCBI")
    });

    assert!(
        has_ncbi_warning,
        "FASTA with invalid NCBI headers should generate warnings"
    );
}
