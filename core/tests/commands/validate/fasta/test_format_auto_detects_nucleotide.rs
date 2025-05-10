use super::utils::create_test_file;
use biolens_core::commands::validate::fasta::{
    options::SequenceType, validator, ValidationOptions,
};

#[test]
fn test_format_auto_detects_nucleotide() {
    let content = ">Sequence1\nACGTN";
    let file = create_test_file(content);

    let options = ValidationOptions {
        sequence_type: SequenceType::Auto,
        ..Default::default()
    };

    let result = validator::validate_fasta(file.path().to_str().unwrap(), options);
    assert!(
        result.is_valid,
        "Valid nucleotide sequence should pass validation"
    );
    assert_eq!(
        result.detected_sequence_type,
        Some(SequenceType::Nucleotide),
        "Sequence type should be auto-detected as nucleotide"
    );
}
