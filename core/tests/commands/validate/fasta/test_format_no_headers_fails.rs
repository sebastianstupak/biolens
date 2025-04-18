use super::utils::create_test_file;
use biolens_core::commands::validate::fasta_validator;

#[test]
fn test_format_no_headers_fails() {
    let content = "ATGCATGCATGCATGCATGC\nGGGCCCAAATTTGGGCCC\nACGTACGTACGTACGT";

    let file = create_test_file(content);

    let result = fasta_validator::validate_fasta(file.path().to_str().unwrap());
    assert!(!result, "FASTA file with no headers should fail validation");
}
