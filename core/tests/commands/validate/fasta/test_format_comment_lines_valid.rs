use super::utils::create_test_file;
use biolens_core::commands::validate::fasta::{validator, ValidationOptions};

#[test]
fn test_format_comment_lines_valid() {
    let content = "; This is a comment line\n>Sequence1\nACGT";
    let file = create_test_file(content);

    let options = ValidationOptions {
        allow_comments: true,
        ..Default::default()
    };

    let result = validator::validate_fasta(file.path().to_str().unwrap(), options);
    assert!(
        result.is_valid,
        "FASTA with comment lines should be valid when comments are allowed"
    );
}
