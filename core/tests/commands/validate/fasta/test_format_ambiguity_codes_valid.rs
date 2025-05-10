use super::utils::create_test_file;
use biolens_core::commands::validate::fasta::{
    options::SequenceType, validator, ValidationOptions,
};

#[test]
fn test_format_ambiguity_codes_valid() {
    // Include all valid nucleotide ambiguity codes
    let content = ">Sequence1\nACGTUNRYKMSWBDHV-.";
    let file = create_test_file(content);

    let options = ValidationOptions {
        sequence_type: SequenceType::Nucleotide,
        ..Default::default()
    };

    let result = validator::validate_fasta(file.path().to_str().unwrap(), options);
    assert!(
        result.is_valid,
        "FASTA with valid ambiguity codes should pass validation"
    );
}
