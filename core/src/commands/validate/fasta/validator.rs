use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{BufRead, BufReader};
use tracing::{debug, info, warn};

use super::options::{SequenceType, ValidationOptions};
use super::result::{IssueLevel, ValidationResult};
use crate::commands::validate::common::Validator;
use crate::commands::validate::formats::{has_valid_extension, FileFormat};

const VALID_NUCLEOTIDES: [char; 19] = [
    'A', 'C', 'G', 'T', 'U', 'I', 'R', 'Y', 'K', 'M', 'S', 'W', 'B', 'D', 'H', 'V', 'N', '-', '.',
];

const VALID_AMINO_ACIDS: [char; 27] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '*',
];

pub struct FastaValidator;

impl Validator for FastaValidator {
    type Options = ValidationOptions;
    type Result = ValidationResult;

    fn build_options_from_args(matches: &clap::ArgMatches) -> Self::Options {
        super::options::build_from_args(matches)
    }

    fn validate(file_path: &str, options: Self::Options) -> Self::Result {
        validate_fasta(file_path, options)
    }
}

/// Determine if a sequence is valid for a given sequence type
fn is_valid_sequence(sequence: &str, seq_type: SequenceType) -> (bool, SequenceType) {
    let check_content = |content: &str, valid_chars: &[char]| -> bool {
        content.chars().all(|c| {
            let upper_c = c.to_uppercase().next().unwrap();
            valid_chars.contains(&upper_c)
        })
    };

    // Check for and reject whitespace within the sequence
    let has_whitespace = sequence
        .chars()
        .any(|c| c.is_whitespace() && c != '\n' && c != '\r');

    // Remove all whitespace for content checking
    let clean_sequence = sequence
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();

    if clean_sequence.is_empty() {
        return (false, seq_type);
    }

    // Return early if whitespace found (not including line breaks)
    if has_whitespace {
        return (false, seq_type);
    }

    match seq_type {
        SequenceType::Nucleotide => (
            check_content(&clean_sequence, &VALID_NUCLEOTIDES),
            SequenceType::Nucleotide,
        ),
        SequenceType::Protein => (
            check_content(&clean_sequence, &VALID_AMINO_ACIDS),
            SequenceType::Protein,
        ),
        SequenceType::Auto => {
            // First try nucleotide as it's more restrictive
            if check_content(&clean_sequence, &VALID_NUCLEOTIDES) {
                return (true, SequenceType::Nucleotide);
            }

            // Then try protein
            if check_content(&clean_sequence, &VALID_AMINO_ACIDS) {
                return (true, SequenceType::Protein);
            }

            // Neither nucleotide nor protein
            (false, SequenceType::Auto)
        }
    }
}

/// Validate NCBI format header
fn is_valid_ncbi_header(header: &str) -> bool {
    // Basic pattern checking for NCBI format identifiers
    let parts: Vec<&str> = header.split('|').collect();
    if parts.len() < 2 {
        return true; // Not using NCBI format, which is still valid
    }

    // Check for known database identifiers
    match parts[0] {
        "gi" => parts.len() >= 2 && !parts[1].is_empty(),
        "lcl" | "bbs" | "bbm" | "gim" => true, // Simple formats
        "gb" | "emb" | "dbj" | "pir" | "sp" | "pdb" | "pat" | "ref" => {
            parts.len() >= 3 && !parts[1].is_empty()
        }
        "gnl" => parts.len() >= 3 && !parts[1].is_empty() && !parts[2].is_empty(),
        _ => true, // Unknown format but we'll consider it valid
    }
}

/// Calculate GC content of a nucleotide sequence
fn calculate_gc_content(sequence: &str) -> f64 {
    let clean_seq = sequence
        .chars()
        .filter(|&c| !c.is_whitespace())
        .map(|c| c.to_uppercase().next().unwrap())
        .collect::<String>();

    if clean_seq.is_empty() {
        return 0.0;
    }

    let gc_count = clean_seq.chars().filter(|&c| c == 'G' || c == 'C').count();
    (gc_count as f64) / (clean_seq.len() as f64) * 100.0
}

/// Main function to validate a FASTA file
pub fn validate_fasta(file_path: &str, options: ValidationOptions) -> ValidationResult {
    let mut result = ValidationResult::new();
    debug!("Validating FASTA file: {}", file_path);

    // Check file extension
    if options.check_extension && !has_valid_extension(file_path, FileFormat::Fasta) {
        result.add_warning(
            format!(
                "File '{}' does not have a standard FASTA extension",
                file_path
            ),
            None,
            None,
        );
    }

    // Open and read the file
    let file = match fs::File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            result.add_error(format!("Failed to open file: {}", err), None, None);
            return result;
        }
    };

    let reader = BufReader::new(file);
    let mut sequences: HashMap<String, String> = HashMap::new();
    let mut cur_header: Option<String> = None;
    let mut line_number = 0;
    let mut detected_types: HashSet<SequenceType> = HashSet::new();

    // Process each line
    for line_result in reader.lines() {
        line_number += 1;

        let line = match line_result {
            Ok(line) => line,
            Err(err) => {
                result.add_error(
                    format!("Error reading line {}: {}", line_number, err),
                    Some(line_number),
                    None,
                );
                return result;
            }
        };

        let trimmed = line.trim();

        // Skip empty lines if allowed
        if trimmed.is_empty() {
            if !options.allow_empty_lines {
                result.add_warning("Empty line found", Some(line_number), cur_header.clone());
            }
            continue;
        }

        // Handle comments
        if trimmed.starts_with(';') {
            if options.allow_comments {
                continue;
            } else {
                result.add_warning(
                    "Comment line found but comments are not allowed",
                    Some(line_number),
                    cur_header.clone(),
                );
                continue;
            }
        }

        // Header line
        if let Some(header_content) = trimmed.strip_prefix(">") {
            // Check if previous header had a sequence
            if let Some(header) = &cur_header {
                if !sequences.contains_key(header) {
                    result.add_error(
                        format!("Header without sequence: {}", header),
                        Some(line_number),
                        Some(header.clone()),
                    );
                }
            }

            // Process new header
            let header = header_content.trim().to_string();

            // Check for empty header
            if header.is_empty() {
                result.add_error("Empty header found", Some(line_number), None);
                cur_header = None;
                continue;
            }

            // Check header length
            if let Some(max_len) = options.max_header_length {
                if header.len() > max_len {
                    result.add_warning(
                        format!("Header exceeds maximum length of {} characters", max_len),
                        Some(line_number),
                        Some(header.clone()),
                    );
                }
            }

            // Check for duplicate headers (always check - this is a standard requirement)
            if sequences.contains_key(&header) {
                result.add_error(
                    format!("Duplicate header found: {}", header),
                    Some(line_number),
                    Some(header.clone()),
                );
            }

            // Validate NCBI header format
            if options.validate_ncbi_headers && !is_valid_ncbi_header(&header) {
                result.add_warning(
                    format!(
                        "Header does not conform to NCBI format standards: {}",
                        header
                    ),
                    Some(line_number),
                    Some(header.clone()),
                );
            }

            cur_header = Some(header.clone());
            sequences.insert(header, String::new());
        } else {
            // Sequence line
            if cur_header.is_none() {
                result.add_error(
                    "Found sequence data without header",
                    Some(line_number),
                    None,
                );
                continue;
            }

            // Check line length
            if options.check_line_length && trimmed.len() > options.max_line_length {
                result.add_warning(
                    format!(
                        "Sequence line exceeds maximum length of {} characters",
                        options.max_line_length
                    ),
                    Some(line_number),
                    cur_header.clone(),
                );
            }

            // Check and handle whitespace in sequence (not allowed in standard FASTA)
            if trimmed.chars().any(|c| c.is_whitespace()) {
                result.add_error(
                    "Whitespace found in sequence line (spaces/tabs are not allowed within sequences)",
                    Some(line_number), cur_header.clone()
                );
            }

            // Remove any whitespace for storage
            let seq_to_add = trimmed
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>();

            // Validate sequence characters
            let (is_valid, detected_type) = is_valid_sequence(&seq_to_add, options.sequence_type);
            if !is_valid {
                result.add_error(
                    format!(
                        "Invalid characters in sequence line for header: {}",
                        cur_header.as_ref().unwrap()
                    ),
                    Some(line_number),
                    cur_header.clone(),
                );
            } else if detected_type != SequenceType::Auto {
                detected_types.insert(detected_type);
            }

            // Add sequence to current header
            let sequence = sequences.get_mut(cur_header.as_ref().unwrap()).unwrap();
            sequence.push_str(&seq_to_add);
        }
    }

    // Check if sequences exist
    if sequences.is_empty() {
        result.add_error("No sequences found in file", None, None);
        return result;
    }

    // Process and validate each sequence
    for (header, sequence) in &mut sequences {
        result.sequence_count += 1;

        // Handle trailing asterisk
        if options.ignore_asterisk_at_end && sequence.ends_with('*') {
            sequence.pop();
        }

        // Handle lowercase/uppercase standardization
        if options.standardize_sequences && !options.allow_lowercase {
            *sequence = sequence.to_uppercase();
        }

        // Check sequence length
        if sequence.is_empty() {
            result.add_error(
                format!("Empty sequence for header: {}", header),
                None,
                Some(header.clone()),
            );
            continue;
        }

        let clean_seq = sequence
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();

        let seq_len = clean_seq.len();

        // Update statistics
        result.total_sequence_length += seq_len;
        result.min_observed_sequence_length = result.min_observed_sequence_length.min(seq_len);
        result.max_observed_sequence_length = result.max_observed_sequence_length.max(seq_len);

        // Check minimum sequence length
        if seq_len < options.min_sequence_length {
            result.add_error(
                format!(
                    "Sequence length ({}) is less than minimum required ({})",
                    seq_len, options.min_sequence_length
                ),
                None,
                Some(header.clone()),
            );
        }

        // Check maximum sequence length if specified
        if let Some(max_len) = options.max_sequence_length {
            if seq_len > max_len {
                result.add_error(
                    format!(
                        "Sequence length ({}) exceeds maximum allowed ({})",
                        seq_len, max_len
                    ),
                    None,
                    Some(header.clone()),
                );
            }
        }

        // Collect GC content for nucleotide sequences
        if options.sequence_type == SequenceType::Nucleotide
            || (options.sequence_type == SequenceType::Auto
                && detected_types.contains(&SequenceType::Nucleotide))
        {
            let gc_content = calculate_gc_content(&clean_seq);
            debug!("Sequence '{}' GC content: {:.2}%", header, gc_content);
        }
    }

    // Determine the detected sequence type
    if detected_types.len() == 1 {
        result.detected_sequence_type = detected_types.iter().next().copied();
    } else if !detected_types.is_empty() {
        result.add_warning("Mixed sequence types detected in file", None, None);
    }

    // Final validity summary
    if result.is_valid {
        info!("FASTA file is valid. Found {} sequences", sequences.len());
    } else {
        warn!(
            "FASTA file is invalid. Found {} issues",
            result
                .issues
                .iter()
                .filter(|i| i.level == IssueLevel::Error)
                .count()
        );
    }

    result
}
