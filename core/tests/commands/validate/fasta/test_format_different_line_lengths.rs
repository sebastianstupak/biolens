use super::utils::create_test_file;
use biolens_core::commands::validate::fasta::{validator, ValidationOptions};

#[test]
fn test_format_different_line_lengths() {
    // Create a FASTA file with sequence lines of different lengths
    let content = ">Sequence1\nAC\nGT\nACGT\nA";
    let file = create_test_file(content);

    let options = ValidationOptions {
        check_line_length: true,
        max_line_length: 80,
        ..Default::default()
    };

    let result = validator::validate_fasta(file.path().to_str().unwrap(), options);

    // Should be valid - sequence lines can have different lengths
    assert!(
        result.is_valid,
        "FASTA with different line lengths should be valid"
    );
    assert_eq!(result.sequence_count, 1, "Should have 1 sequence");
    assert_eq!(
        result.total_sequence_length, 9,
        "Total sequence length should be 9bp"
    );
}
