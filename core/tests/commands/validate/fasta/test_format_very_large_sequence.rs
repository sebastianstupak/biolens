use super::utils::create_test_file;
use biolens_core::commands::validate::fasta::{
    options::SequenceType, validator, ValidationOptions,
};

#[test]
fn test_format_very_large_sequence() {
    // Create a large sequence (10,000 nucleotides)
    let large_sequence = "ACGT".repeat(2500);
    let content = format!(">Large_Sequence\n{}", large_sequence);
    let file = create_test_file(&content);

    let options = ValidationOptions {
        sequence_type: SequenceType::Nucleotide,
        ..Default::default()
    };

    let result = validator::validate_fasta(file.path().to_str().unwrap(), options);

    // Debug output
    for issue in &result.issues {
        println!("Issue: {:?} - {}", issue.level, issue.message);
    }

    // Large sequences should still be valid
    assert!(result.is_valid, "FASTA with large sequence should be valid");

    // Check that the sequence length is correctly reported
    assert_eq!(result.sequence_count, 1, "Should have 1 sequence");
    assert_eq!(
        result.total_sequence_length, 10000,
        "Sequence length should be 10000"
    );
    assert_eq!(
        result.max_observed_sequence_length, 10000,
        "Max sequence length should be 10000"
    );
}
