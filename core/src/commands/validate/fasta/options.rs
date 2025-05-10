#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SequenceType {
    Nucleotide,
    Protein,
    Auto,
}

#[derive(Debug, Clone)]
pub struct ValidationOptions {
    // Sequence validation options
    pub sequence_type: SequenceType,
    pub check_line_length: bool,
    pub max_line_length: usize,
    pub min_sequence_length: usize,
    pub max_sequence_length: Option<usize>,
    pub allow_lowercase: bool,
    pub ignore_asterisk_at_end: bool,
    pub standardize_sequences: bool,

    // Header validation options
    pub validate_ncbi_headers: bool,
    pub max_header_length: Option<usize>,

    // File validation options
    pub check_extension: bool,
    pub allow_empty_lines: bool,
    pub allow_comments: bool,
}

impl Default for ValidationOptions {
    fn default() -> Self {
        ValidationOptions {
            sequence_type: SequenceType::Auto,
            check_line_length: true,
            max_line_length: 80,
            min_sequence_length: 1,
            max_sequence_length: None,
            allow_lowercase: true,
            ignore_asterisk_at_end: true,
            standardize_sequences: false,
            validate_ncbi_headers: false,
            max_header_length: None,
            check_extension: true,
            allow_empty_lines: true,
            allow_comments: true,
        }
    }
}

/// Build FASTA ValidationOptions from command-line arguments
pub fn build_from_args(matches: &clap::ArgMatches) -> ValidationOptions {
    let mut options = ValidationOptions::default();

    // Sequence type
    if let Some(seq_type) = matches.get_one::<String>("sequence-type") {
        options.sequence_type = match seq_type.as_str() {
            "nucleotide" => SequenceType::Nucleotide,
            "protein" => SequenceType::Protein,
            _ => SequenceType::Auto,
        };
    }

    // Line length checking
    options.check_line_length = !matches.get_flag("no-line-length-check");
    if let Some(max_len) = matches.get_one::<usize>("max-line-length") {
        options.max_line_length = *max_len;
    }

    // Sequence length
    if let Some(min_len) = matches.get_one::<usize>("min-sequence-length") {
        options.min_sequence_length = *min_len;
    }

    if let Some(max_len) = matches.get_one::<usize>("max-sequence-length") {
        options.max_sequence_length = Some(*max_len);
    }

    // Lowercase handling
    options.allow_lowercase = !matches.get_flag("disallow-lowercase");

    // Asterisk handling
    options.ignore_asterisk_at_end = !matches.get_flag("keep-asterisk");

    // Standardization
    options.standardize_sequences = matches.get_flag("standardize");

    // NCBI validation
    options.validate_ncbi_headers = matches.get_flag("validate-ncbi");

    // Header length
    if let Some(max_len) = matches.get_one::<usize>("max-header-length") {
        options.max_header_length = Some(*max_len);
    }

    // File extension checking
    options.check_extension = !matches.get_flag("no-extension-check");

    // Empty lines and comments
    options.allow_empty_lines = !matches.get_flag("disallow-empty-lines");
    options.allow_comments = !matches.get_flag("disallow-comments");

    options
}
