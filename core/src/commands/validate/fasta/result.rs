use super::options::SequenceType;
use crate::commands::validate::common::ValidationResultTrait;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IssueLevel {
    Error,   // Prevents the file from being valid
    Warning, // Suboptimal but not invalid
    Info,    // Informational only
}

#[derive(Debug, Clone)]
pub struct ValidationIssue {
    pub level: IssueLevel,
    pub message: String,
    pub line_number: Option<usize>,
    pub header: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub issues: Vec<ValidationIssue>,
    pub sequence_count: usize,
    pub total_sequence_length: usize,
    pub min_observed_sequence_length: usize,
    pub max_observed_sequence_length: usize,
    pub detected_sequence_type: Option<SequenceType>,
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self {
            is_valid: true,
            issues: Vec::new(),
            sequence_count: 0,
            total_sequence_length: 0,
            min_observed_sequence_length: usize::MAX,
            max_observed_sequence_length: 0,
            detected_sequence_type: None,
        }
    }
}

impl ValidationResult {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_issue(
        &mut self,
        level: IssueLevel,
        message: impl Into<String>,
        line_number: Option<usize>,
        header: Option<String>,
    ) {
        let message = message.into();
        if level == IssueLevel::Error {
            self.is_valid = false;
        }
        self.issues.push(ValidationIssue {
            level,
            message,
            line_number,
            header,
        });
    }

    pub fn add_error(
        &mut self,
        message: impl Into<String>,
        line_number: Option<usize>,
        header: Option<String>,
    ) {
        self.add_issue(IssueLevel::Error, message, line_number, header);
    }

    pub fn add_warning(
        &mut self,
        message: impl Into<String>,
        line_number: Option<usize>,
        header: Option<String>,
    ) {
        self.add_issue(IssueLevel::Warning, message, line_number, header);
    }

    pub fn add_info(
        &mut self,
        message: impl Into<String>,
        line_number: Option<usize>,
        header: Option<String>,
    ) {
        self.add_issue(IssueLevel::Info, message, line_number, header);
    }
}

impl ValidationResultTrait for ValidationResult {
    fn is_valid(&self) -> bool {
        self.is_valid
    }

    fn print_summary(&self) {
        super::summary::print_summary(self);
    }

    fn print_minimal(&self) {
        println!("{}", if self.is_valid { "valid" } else { "invalid" });
    }
}
