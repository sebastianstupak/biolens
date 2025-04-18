use super::utils::create_test_file;
use biolens_core::commands::validate::fasta_validator;

#[test]
fn test_format_multiple_sequences_valid() {
    let content = ">Sequence1\nACGTAGCT\n>Sequence2\nGTCAAAACGT\n>Sequence3\nTTTGGGCCCAA";
    let file = create_test_file(content);

    let result = fasta_validator::validate_fasta(file.path().to_str().unwrap());
    assert!(
        result,
        "Valid FASTA file with multiple sequences should pass validation"
    );
}
