use super::utils::create_test_file;
use biolens_core::commands::validate::fasta_validator;

#[test]
fn test_format_empty_file_fails() {
    let content = "";
    let file = create_test_file(content);

    let result = fasta_validator::validate_fasta(file.path().to_str().unwrap());
    assert!(!result, "Empty FASTA file should fail validation");
}
