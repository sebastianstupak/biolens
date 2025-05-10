use super::utils::create_test_file;
use biolens_core::commands::validate::fasta::{
    options::SequenceType, validator, ValidationOptions,
};

#[test]
fn test_format_lowercase_nucleotides_valid() {
    let content = ">Sequence1\nacgt"; // Lowercase nucleotides
    let file = create_test_file(content);

    let options = ValidationOptions {
        sequence_type: SequenceType::Nucleotide,
        allow_lowercase: true, // Allow lowercase letters
        ..Default::default()
    };

    let result = validator::validate_fasta(file.path().to_str().unwrap(), options);
    assert!(
        result.is_valid,
        "FASTA with lowercase nucleotides should be valid"
    );
}
