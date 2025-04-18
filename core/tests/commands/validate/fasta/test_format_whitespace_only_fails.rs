use super::utils::create_test_file;
use biolens_core::commands::validate::fasta_validator;

#[test]
fn test_format_whitespace_only_fails() {
    let content = "   ";
    let file = create_test_file(content);

    let result = fasta_validator::validate_fasta(file.path().to_str().unwrap());
    assert!(
        !result,
        "FASTA file with only whitespaces should fail validation"
    );
}
