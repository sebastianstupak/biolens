use super::utils::create_test_file;
use biolens_core::commands::validate::fasta::{
    options::SequenceType, validator, ValidationOptions,
};

#[test]
fn test_format_sequence_with_numbers_fails() {
    // Some FASTA files incorrectly include position numbers in sequence
    let content = ">Sequence1\n10 ACGTACGTAC 20\n30 GGGCCCAAAT 40";
    let file = create_test_file(content);

    let options = ValidationOptions {
        sequence_type: SequenceType::Nucleotide,
        ..Default::default()
    };

    let result = validator::validate_fasta(file.path().to_str().unwrap(), options);

    // Should fail validation due to numbers and spaces
    assert!(
        !result.is_valid,
        "FASTA with numbers in sequence should fail validation"
    );
}
