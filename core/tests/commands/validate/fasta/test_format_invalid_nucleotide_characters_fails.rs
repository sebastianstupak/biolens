use super::utils::create_test_file;
use biolens_core::commands::validate::fasta::validator;
use biolens_core::commands::validate::fasta::{options::SequenceType, ValidationOptions};

#[test]
fn test_format_invalid_nucleotide_characters_fails() {
    let content = ">Sequence1\nACGTXZ123";
    let file = create_test_file(content);

    let options = ValidationOptions {
        sequence_type: SequenceType::Nucleotide,
        ..Default::default()
    };

    let result = validator::validate_fasta(file.path().to_str().unwrap(), options);
    assert!(
        !result.is_valid,
        "FASTA with invalid nucleotide characters should fail validation"
    );
}
