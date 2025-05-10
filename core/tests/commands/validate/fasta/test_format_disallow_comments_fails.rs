use super::utils::create_test_file;
use biolens_core::commands::validate::fasta::{validator, ValidationOptions};

#[test]
fn test_format_disallow_comments_fails() {
    let content = "; This is a comment line\n>Sequence1\nACGT";
    let file = create_test_file(content);

    let options = ValidationOptions {
        allow_comments: false,
        ..Default::default()
    };

    let result = validator::validate_fasta(file.path().to_str().unwrap(), options);
    // This should cause a warning, not an error, so check if warnings exist
    assert!(
        !result.issues.is_empty(),
        "FASTA with comment lines should have warnings when comments are disallowed"
    );
}
