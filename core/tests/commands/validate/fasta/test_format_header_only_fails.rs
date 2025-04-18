use super::utils::create_test_file;
use biolens_core::commands::validate::fasta_validator;

#[test]
fn test_format_header_only_fails() {
    let content = ">Sequence1";
    let file = create_test_file(content);

    let result = fasta_validator::validate_fasta(file.path().to_str().unwrap());
    assert!(
        !result,
        "FASTA file with only header should fail validation"
    );
}
