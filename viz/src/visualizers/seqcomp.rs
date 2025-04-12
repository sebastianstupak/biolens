use biolens_common::seqcomp::ComparisonResult;

pub fn visualize(result: &ComparisonResult) {
    println!("Generating sequence comparison visualization:");
    println!(" Sequence 1: {}", result.file1);
    println!(" Sequence 2: {}", result.file2);

    println!(" TODO: Alignment visualization")
}
