use super::utils::create_test_file;
use biolens_core::commands::validate::fasta::{validator, ValidationOptions};

#[test]
fn test_format_repeated_sequence_names() {
    // Create a FASTA file with sequences that have the same description
    // but different IDs (which is valid according to spec)
    let content = ">gi|123|gb|ABC| Sample sequence\nACGT\n>gi|456|gb|DEF| Sample sequence\nGGGG";
    let file = create_test_file(content);

    let options = ValidationOptions::default();
    let result = validator::validate_fasta(file.path().to_str().unwrap(), options);

    // This should be valid - different IDs with same description are allowed
    assert!(
        result.is_valid,
        "FASTA with repeated sequence descriptions should be valid"
    );
    assert_eq!(result.sequence_count, 2, "Should have 2 sequences");
}
