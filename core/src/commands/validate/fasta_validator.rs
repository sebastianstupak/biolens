use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use tracing::debug;

const VALID_NUCLEOTIDES: [char; 11] = ['A', 'C', 'G', 'T', 'U', 'N', 'R', 'Y', 'S', 'W', '-'];

fn is_valid_sequence(sequence: &str) -> bool {
    sequence.chars().all(|c| {
        let upper_c = c.to_uppercase().next().unwrap();
        VALID_NUCLEOTIDES.contains(&upper_c)
    })
}

pub fn validate_fasta(file_path: &str) -> bool {
    // TODO:
    // - file size check
    // - line length consistency
    // - additional char validation
    //   - check for lowercase vs uppercase nucleotides (standardize if needed)
    //   - support for protein sequences with amino acid code
    //   - handle ambiguity codes like K, M, B, D, H, V for nucleotides
    // - whitespaces within sequences
    // - header format validation
    //   - max length
    //   - valid char set
    //   - proper structure if following a standard format (e.g., NCBI format like >gi|12345|ref|NC_123456.1|)
    // - comment handling
    // - end-of-line character standardization - check for and handle mixed line endings (CR, LF, CRLF)
    // - special/control characters detection - check for any invisible control characters that might cause issues
    // - multi-line header handling - Some non-standard FASTA files might have headers spanning multiple lines
    // - circular sequence annotation - Check for or handle annotations indicating circular sequences

    debug!("Validating FASTA file: {}", file_path);
    let file = match fs::File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            debug!("Failed to open file: {}", err);
            return false;
        }
    };

    let reader = BufReader::new(file);
    let mut sequences: HashMap<String, String> = HashMap::new();
    let mut cur_header: Option<String> = None;

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(err) => {
                debug!("Error reading line: {}", err);
                return false;
            }
        };

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if let Some(header_content) = trimmed.strip_prefix(">") {
            if let Some(header) = &cur_header {
                if !sequences.contains_key(header) {
                    debug!("Found header without sequence: {}", header);
                    return false;
                }
            }

            let header = header_content.trim().to_string();
            if header.is_empty() {
                debug!("Found empty header");
                return false;
            }

            if sequences.contains_key(&header) {
                debug!("Duplicate header found: {}", header);
                return false;
            }

            cur_header = Some(header);
            sequences.insert(cur_header.as_ref().unwrap().clone(), String::new());
        } else {
            if cur_header.is_none() {
                debug!("Found sequence data without header");
                return false;
            }

            if !is_valid_sequence(trimmed) {
                debug!(
                    "Invalid characters in sequence for header: {}",
                    cur_header.as_ref().unwrap()
                );
                return false;
            }

            let sequence = sequences.get_mut(cur_header.as_ref().unwrap()).unwrap();
            sequence.push_str(trimmed);
        }
    }

    if sequences.is_empty() {
        debug!("No headers found in file");
        return false;
    }

    for (header, sequence) in &sequences {
        if sequence.is_empty() {
            debug!("Empty sequence for header: {}", header);
            return false;
        }
    }

    debug!("FASTA file is valid. Found {} sequences", sequences.len());
    true
}
