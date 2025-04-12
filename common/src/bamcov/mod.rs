#[derive(Debug, Clone)]
pub struct CoverageResult {
    pub file_path: String,
    pub region: Option<String>,
}
