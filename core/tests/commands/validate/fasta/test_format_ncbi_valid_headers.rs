use super::utils::create_test_file;
use biolens_core::commands::validate::fasta::{validator, ValidationOptions};

#[test]
fn test_format_ncbi_valid_headers() {
    let content = ">gi|12345|gb|AAD44166.1| cytochrome b [Elephas maximus maximus]\nACGT";
    let file = create_test_file(content);

    let options = ValidationOptions {
        validate_ncbi_headers: true,
        ..Default::default()
    };

    let result = validator::validate_fasta(file.path().to_str().unwrap(), options);
    assert!(
        result.is_valid,
        "FASTA with valid NCBI headers should pass validation"
    );
}
