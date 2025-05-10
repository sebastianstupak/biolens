use clap::ArgMatches;
use std::str::FromStr;
use std::time::Instant;
use tracing::{debug, error, info, instrument, trace, warn};

use super::common::Validator;
use super::fasta::FastaValidator;
use super::formats::{detect_format_from_path, FileFormat};

/// Main validation function that dispatches to format-specific validators
#[instrument(skip_all, fields(file_path = %file_path))]
pub fn validate(file_path: &str, matches: &ArgMatches, format_override: Option<&str>) -> bool {
    debug!("Starting validation process");

    let format = if let Some(fmt) = format_override {
        FileFormat::from_str(fmt).unwrap_or(FileFormat::Unknown)
    } else {
        detect_format_from_path(file_path)
    };

    if format == FileFormat::Unknown {
        error!(file = %file_path, "Unknown file format");
        println!("Unknown file format. Use --format to specify the format explicitly.");
        return false;
    }

    debug!(format = ?format, "File format determined");

    let start_time = Instant::now();
    let result = match format {
        FileFormat::Fasta => {
            trace!("Delegating to FASTA validator");

            let options = FastaValidator::build_options_from_args(matches);
            let validation_result = FastaValidator::validate(file_path, options);
            FastaValidator::print_results(&validation_result)
        }
        FileFormat::Sam => {
            trace!("Delegating to SAM validator");

            // TODO:
            println!("Validating SAM file: {}", file_path);
            let is_valid = super::sam::SamValidator::validate(file_path);

            if is_valid {
                println!("SAM file is valid");
            } else {
                println!("SAM file is invalid");
            }

            is_valid
        }
        format if !format.is_implemented() => {
            let message = format!("{} validation not yet implemented", format.as_str());
            warn!(message = %message, "{}", message);
            println!("WARNING: {}", message);
            false
        }
        _ => {
            let message = format!("Unsupported file format: {}", format.as_str());
            error!(format = ?format, "{}", message);
            println!("ERROR: {}", message);
            false
        }
    };

    let elapsed = start_time.elapsed();
    info!(
        success = result,
        duration_ms = elapsed.as_millis(),
        "Validation completed in {}ms",
        elapsed.as_millis()
    );

    result
}
