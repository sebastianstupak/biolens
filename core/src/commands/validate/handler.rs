use super::fasta_validator;
use super::sam_validator;
use std::path::Path;

pub fn validate(file_path: &str) -> bool {
    println!("Validating file: {}", file_path);

    let path = Path::new(file_path);
    if path.extension().is_none() {
        println!("Error: File has no extension!");
        return false;
    }

    let extension = path.extension().unwrap().to_string_lossy().to_lowercase();

    match extension.as_str() {
        "sam" => sam_validator::validate_sam(file_path),
        "bam" => {
            println!("BAM validation not yet implemented");
            false
        }
        "fa" | "fasta" | "fna" | "faa" | "ffn" | "frn" | "fas" | "seq" | "mpfa" | "csfasta"
        | "pfa" => fasta_validator::validate_fasta(file_path),
        "fq" | "fastq" => {
            println!("FASTQ validation not yet implemented");
            false
        }
        "vcf" => {
            println!("VCF validation not yet implemented");
            false
        }
        "bed" => {
            println!("BED validation not yet implemented");
            false
        }
        "gff" | "gff3" => {
            println!("GFF validation not yet implemented");
            false
        }
        "gtf" => {
            println!("GTF validation not yet implemented");
            false
        }
        _ => {
            println!("Unsupported file format: {}", extension);
            false
        }
    }
}
