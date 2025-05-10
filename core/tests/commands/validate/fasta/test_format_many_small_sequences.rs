use super::utils::create_test_file;
use biolens_core::commands::validate::fasta::{
    options::SequenceType, validator, ValidationOptions,
};

#[test]
fn test_format_many_small_sequences() {
    // Create a file with many small sequences (100 sequences of 10bp each)
    let mut content = String::new();
    for i in 1..=100 {
        content.push_str(&format!(">Sequence{}\nACGTACGTAC\n", i));
    }

    let file = create_test_file(&content);

    let options = ValidationOptions {
        sequence_type: SequenceType::Nucleotide,
        ..Default::default()
    };

    let result = validator::validate_fasta(file.path().to_str().unwrap(), options);

    // Debug output if any issues
    for issue in &result.issues {
        println!("Issue: {:?} - {}", issue.level, issue.message);
    }

    // Many small sequences should be valid
    assert!(
        result.is_valid,
        "FASTA with many small sequences should be valid"
    );

    // Check that all sequences are counted correctly
    assert_eq!(result.sequence_count, 100, "Should have 100 sequences");
    assert_eq!(
        result.total_sequence_length, 1000,
        "Total sequence length should be 1000"
    );
    assert_eq!(
        result.min_observed_sequence_length, 10,
        "Min sequence length should be 10"
    );
    assert_eq!(
        result.max_observed_sequence_length, 10,
        "Max sequence length should be 10"
    );
}
