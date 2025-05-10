use super::utils::create_test_file;
use biolens_core::commands::validate::fasta::{
    options::SequenceType, validator, ValidationOptions,
};

#[test]
fn test_format_invalid_protein_characters_fails() {
    let content = ">Sequence1\nAVPQR123";
    let file = create_test_file(content);

    let options = ValidationOptions {
        sequence_type: SequenceType::Protein,
        ..Default::default()
    };

    let result = validator::validate_fasta(file.path().to_str().unwrap(), options);
    assert!(
        !result.is_valid,
        "FASTA with invalid protein characters should fail validation"
    );
}
