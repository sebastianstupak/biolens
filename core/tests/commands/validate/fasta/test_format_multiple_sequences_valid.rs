use super::utils::create_test_file;
use biolens_core::commands::validate::fasta::{validator, ValidationOptions};

#[test]
fn test_format_multiple_sequences_valid() {
    let content = ">Sequence1\nACGTAGCT\n>Sequence2\nGTCAAAACGT\n>Sequence3\nTTTGGGCCCAA";
    let file = create_test_file(content);
    let options = ValidationOptions::default();
    let validation_result = validator::validate_fasta(file.path().to_str().unwrap(), options);

    assert!(
        validation_result.is_valid,
        "Valid FASTA file with multiple sequences should pass validation"
    );
}
