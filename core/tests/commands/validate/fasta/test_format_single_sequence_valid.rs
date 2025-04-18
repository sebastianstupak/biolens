use super::utils::create_test_file;
use biolens_core::commands::validate::fasta_validator;

#[test]
fn test_format_single_sequence_valid() {
    let content = ">Sequence1\nACGT\n";
    let file = create_test_file(content);

    let result = fasta_validator::validate_fasta(file.path().to_str().unwrap());
    assert!(
        result,
        "Valid FASTA file with single sequence should pass validation"
    );
}
