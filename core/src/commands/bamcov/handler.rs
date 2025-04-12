use biolens_common::bamcov::CoverageResult;

pub fn calculate_coverage(bam_path: &str, region: Option<&str>) -> CoverageResult {
  println!("Analyzing BAM coverage for: {}", bam_path);
  if let Some(reg) = region {
      println!("  Region: {}", reg);
  } else {
      println!("  Region: whole genome");
  }

  CoverageResult {
      file_path: bam_path.to_string(),
      region: region.map(String::from),
  }
}

pub fn visualize_coverage(_result: &CoverageResult) {
  #[cfg(feature = "visualization")]
  biolens_viz::visualizers::bamcov::visualize(_result);
}
