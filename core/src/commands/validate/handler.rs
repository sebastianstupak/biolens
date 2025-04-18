use super::fasta_validator;
use super::sam_validator;
use log::{debug, error, info, warn};
use std::path::Path;

pub fn validate(file_path: &str) -> bool {
    info!("Validating file: {}", file_path);

    let path = Path::new(file_path);
    if path.extension().is_none() {
        error!("File has no extension: {}", file_path);
        return false;
    }

    let extension = path.extension().unwrap().to_string_lossy().to_lowercase();
    debug!("File extension detected: {}", extension);

    match extension.as_str() {
        "sam" => {
            debug!("Processing as SAM file");
            sam_validator::validate_sam(file_path)
        }
        "bam" => {
            warn!("BAM validation not yet implemented");
            false
        }
        "fa" | "fasta" | "fna" | "faa" | "ffn" | "frn" | "fas" | "seq" | "mpfa" | "csfasta"
        | "pfa" => {
            debug!("Processing as FASTA file");
            fasta_validator::validate_fasta(file_path)
        }
        "fq" | "fastq" => {
            warn!("FASTQ validation not yet implemented");
            false
        }
        "vcf" => {
            warn!("VCF validation not yet implemented");
            false
        }
        "bed" => {
            warn!("BED validation not yet implemented");
            false
        }
        "gff" | "gff3" => {
            warn!("GFF validation not yet implemented");
            false
        }
        "gtf" => {
            warn!("GTF validation not yet implemented");
            false
        }
        _ => {
            error!("Unsupported file format: {}", extension);
            false
        }
    }
}
