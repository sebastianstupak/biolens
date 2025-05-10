use super::utils::create_test_file;
use biolens_core::commands::validate::fasta::{validator, ValidationOptions};

#[test]
fn test_format_with_semicolon_headers() {
    // In original FASTA format, semicolon could be used for headers
    let content = ";Sequence1\nACGT\n;Sequence2\nGGGG";
    let file = create_test_file(content);

    // Test with comments allowed
    let options = ValidationOptions {
        allow_comments: true,
        ..Default::default()
    };

    let result = validator::validate_fasta(file.path().to_str().unwrap(), options);

    // This should either be treated as comments or as sequences
    // Let's check the behavior - if it's comments, we should have 0 sequences
    if result.sequence_count == 0 {
        // If they were treated as comments, the file should fail validation due to no sequences
        assert!(
            !result.is_valid,
            "FASTA with only semicolon headers should fail if treated as comments"
        );
    } else {
        // If they were treated as headers, we should have 2 sequences
        assert_eq!(
            result.sequence_count, 2,
            "Should have 2 sequences if semicolon headers are supported"
        );
        assert!(
            result.is_valid,
            "FASTA with semicolon headers should be valid if they're supported"
        );
    }
}
