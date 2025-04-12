use biolens_common::seqcomp::ComparisonResult;

pub fn compare_sequences(file1: &str, file2: &str) -> ComparisonResult {
    println!("Comparing sequences:");
    println!("  Sequence 1: {}", file1);
    println!("  Sequence 2: {}", file2);

    ComparisonResult {
        file1: file1.to_string(),
        file2: file2.to_string(),
    }
}

pub fn visualize_comparison(_result: &ComparisonResult) {
    #[cfg(feature = "visualization")]
    biolens_viz::visualizers::seqcomp::visualize(_result);
}
