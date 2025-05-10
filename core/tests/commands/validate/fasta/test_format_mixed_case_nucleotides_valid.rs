use super::utils::create_test_file;
use biolens_core::commands::validate::fasta::{
    options::SequenceType, validator, ValidationOptions,
};

#[test]
fn test_format_mixed_case_nucleotides_valid() {
    let content = ">Sequence1\nAcgTnN";
    let file = create_test_file(content);

    let options = ValidationOptions {
        sequence_type: SequenceType::Nucleotide,
        allow_lowercase: true,
        ..Default::default()
    };

    let result = validator::validate_fasta(file.path().to_str().unwrap(), options);
    assert!(
        result.is_valid,
        "FASTA with mixed case nucleotides should be valid"
    );
}
