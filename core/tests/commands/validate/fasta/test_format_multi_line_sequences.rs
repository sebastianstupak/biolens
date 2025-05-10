use super::utils::create_test_file;
use biolens_core::commands::validate::fasta::{validator, ValidationOptions};

#[test]
fn test_format_multi_line_sequences() {
    // Create a sequence split across multiple lines
    let content = ">Sequence1\nACGT\nACGT\nACGT\n>Sequence2\nGGGG";
    let file = create_test_file(content);

    let options = ValidationOptions::default();
    let result = validator::validate_fasta(file.path().to_str().unwrap(), options);

    // Should be valid - sequences can span multiple lines
    assert!(
        result.is_valid,
        "FASTA with multi-line sequences should be valid"
    );
    assert_eq!(result.sequence_count, 2, "Should have 2 sequences");

    // First sequence should be 12bp (4bp × 3 lines)
    assert_eq!(
        result.total_sequence_length, 16,
        "Total sequence length should be 16bp"
    );
}
