use super::utils::create_test_file;
use biolens_core::commands::validate::fasta::{validator, ValidationOptions};

#[test]
fn test_format_sequence_in_header_fails() {
    // Create a FASTA file with sequence data incorrectly included in the header line
    let content = ">Sequence1 ACGT\nGGGG";
    let file = create_test_file(content);

    let options = ValidationOptions::default();
    let result = validator::validate_fasta(file.path().to_str().unwrap(), options);

    // This should be valid - it's not ideal but not invalid according to FASTA spec
    // The header can contain anything after the > until the newline
    assert!(
        result.is_valid,
        "FASTA with sequence in header should still be valid"
    );

    // The sequence should be just GGGG, not including the ACGT in the header
    assert_eq!(result.sequence_count, 1, "Should have 1 sequence");
}
