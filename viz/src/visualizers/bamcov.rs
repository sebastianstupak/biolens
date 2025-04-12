use biolens_common::bamcov::CoverageResult;

pub fn visualize(result: &CoverageResult) {
    println!(
        "Generating BAM coverage visualization for: {}",
        result.file_path
    );
    if let Some(region) = &result.region {
        println!(" Visualizing region: {}", region);
    } else {
        println!(" Visualizing whole genome coverage");
    }

    println!(" TODO: Coverage plot here")
}
