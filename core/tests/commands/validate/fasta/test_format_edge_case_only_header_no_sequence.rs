use super::utils::create_test_file;
use biolens_core::commands::validate::fasta::{validator, ValidationOptions};

#[test]
fn test_format_edge_case_only_header_no_sequence() {
    // Create a minimal FASTA file with just a header
    let content = ">Sequence1"; // No sequence at all
    let file = create_test_file(content);

    let options = ValidationOptions::default();
    let result = validator::validate_fasta(file.path().to_str().unwrap(), options);

    // Debug output to understand what's happening
    println!("Is valid: {}", result.is_valid);
    for issue in &result.issues {
        println!("Issue: {:?} - {}", issue.level, issue.message);
    }

    // This should fail - headers must have sequences
    assert!(
        !result.is_valid,
        "FASTA with header but no sequence should fail validation"
    );

    // The error message might be different than expected
    // Let's check for any error message about headers or sequences
    let has_error_about_header_or_sequence = result.issues.iter().any(|issue| {
        issue.message.contains("header")
            || issue.message.contains("Header")
            || issue.message.contains("sequence")
            || issue.message.contains("Sequence")
    });

    assert!(
        has_error_about_header_or_sequence,
        "Should report some error related to headers or sequences"
    );
}
