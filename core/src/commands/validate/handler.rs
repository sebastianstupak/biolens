use super::fasta_validator;
use super::sam_validator;
use std::path::Path;
use std::time::Instant;
use tracing::{debug, error, info, instrument, trace, warn};

#[instrument(skip_all, fields(file_path = %file_path))]
pub fn validate(file_path: &str) -> bool {
    debug!("Starting validation process");

    let path = Path::new(file_path);
    if path.extension().is_none() {
        error!(file = %file_path, "File has no extension");
        return false;
    }

    let extension = path.extension().unwrap().to_string_lossy().to_lowercase();
    debug!(extension = %extension, "File extension detected");

    let start_time = Instant::now();
    let result = match extension.as_str() {
        "sam" => {
            trace!("Delegating to SAM validator");
            sam_validator::validate_sam(file_path)
        }
        "bam" => {
            warn!(
                message = "Not implemented",
                "BAM validation not yet implemented"
            );
            false
        }
        "fa" | "fasta" | "fna" | "faa" | "ffn" | "frn" | "fas" | "seq" | "mpfa" | "csfasta"
        | "pfa" => {
            trace!("Delegating to FASTA validator");
            fasta_validator::validate_fasta(file_path)
        }
        "fq" | "fastq" => {
            warn!(
                message = "Not implemented",
                "FASTQ validation not yet implemented"
            );
            false
        }
        "vcf" => {
            warn!(
                message = "Not implemented",
                "VCF validation not yet implemented"
            );
            false
        }
        "bed" => {
            warn!(
                message = "Not implemented",
                "BED validation not yet implemented"
            );
            false
        }
        "gff" | "gff3" => {
            warn!(
                message = "Not implemented",
                "GFF validation not yet implemented"
            );
            false
        }
        "gtf" => {
            warn!(
                message = "Not implemented",
                "GTF validation not yet implemented"
            );
            false
        }
        _ => {
            error!(format = %extension, "Unsupported file format");
            false
        }
    };

    let elapsed = start_time.elapsed();
    info!(
        success = result,
        duration_ms = elapsed.as_millis(),
        "Validation completed"
    );

    result
}
