use super::utils::create_test_file;
use biolens_core::commands::validate::fasta::{
    options::SequenceType, validator, ValidationOptions,
};

#[test]
fn test_format_gc_content_calculation() {
    // This test doesn't directly verify GC content calculation since it's not returned
    // But it ensures that GC content calculation doesn't cause any errors

    // Create a sequence with known GC content (50%)
    let content = ">Sequence1\nACGTACGTACGT"; // 50% GC content
    let file = create_test_file(content);

    let options = ValidationOptions {
        sequence_type: SequenceType::Nucleotide,
        ..Default::default()
    };

    let result = validator::validate_fasta(file.path().to_str().unwrap(), options);

    // Validation should succeed
    assert!(result.is_valid, "FASTA with 50% GC content should be valid");

    // We can't directly test the GC calculation since it's not returned
    // But this test at least verifies it doesn't cause errors
}
