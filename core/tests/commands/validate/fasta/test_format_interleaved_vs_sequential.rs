use super::utils::create_test_file;
use biolens_core::commands::validate::fasta::{validator, ValidationOptions};

#[test]
fn test_format_interleaved_vs_sequential() {
    // Test both interleaved and sequential formats

    // Sequential format (all sequences one after another)
    let sequential = ">Seq1\nACGT\n>Seq2\nGGGG\n>Seq3\nTTTT";
    let seq_file = create_test_file(sequential);

    // Interleaved format (sequences broken into blocks)
    // This isn't true interleaved format (like in MSA), but tests multi-line sequences
    let interleaved = ">Seq1\nAC\nGT\n>Seq2\nGG\nGG\n>Seq3\nTT\nTT";
    let inter_file = create_test_file(interleaved);

    let options = ValidationOptions::default();

    let seq_result = validator::validate_fasta(seq_file.path().to_str().unwrap(), options.clone());
    let inter_result = validator::validate_fasta(inter_file.path().to_str().unwrap(), options);

    // Both formats should be valid
    assert!(
        seq_result.is_valid,
        "Sequential FASTA format should be valid"
    );
    assert!(
        inter_result.is_valid,
        "Interleaved FASTA format should be valid"
    );

    // Both should have 3 sequences
    assert_eq!(
        seq_result.sequence_count, 3,
        "Sequential format should have 3 sequences"
    );
    assert_eq!(
        inter_result.sequence_count, 3,
        "Interleaved format should have 3 sequences"
    );

    // Total sequence length should be the same (12bp)
    assert_eq!(
        seq_result.total_sequence_length, 12,
        "Sequential format should have 12bp total"
    );
    assert_eq!(
        inter_result.total_sequence_length, 12,
        "Interleaved format should have 12bp total"
    );
}
