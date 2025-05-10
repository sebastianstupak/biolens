use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fmt;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileFormat {
    Fasta,
    Sam,
    Bam,
    Fastq,
    Vcf,
    Bed,
    Gff,
    Gtf,
    Unknown,
}

#[derive(Debug)]
pub struct ParseFormatError;

impl fmt::Display for ParseFormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown file format")
    }
}

impl FromStr for FileFormat {
    type Err = ParseFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "fasta" => FileFormat::Fasta,
            "fas" | "fa" | "fna" | "ffn" | "faa" | "mpfa" | "frn" => FileFormat::Fasta,
            "sam" => FileFormat::Sam,
            "bam" => FileFormat::Bam,
            "fastq" | "fq" => FileFormat::Fastq,
            "vcf" => FileFormat::Vcf,
            "bed" => FileFormat::Bed,
            "gff" | "gff3" => FileFormat::Gff,
            "gtf" => FileFormat::Gtf,
            _ => FileFormat::Unknown,
        })
    }
}

impl FileFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            FileFormat::Fasta => "FASTA",
            FileFormat::Sam => "SAM",
            FileFormat::Bam => "BAM",
            FileFormat::Fastq => "FASTQ",
            FileFormat::Vcf => "VCF",
            FileFormat::Bed => "BED",
            FileFormat::Gff => "GFF",
            FileFormat::Gtf => "GTF",
            FileFormat::Unknown => "UNKNOWN",
        }
    }

    pub fn is_implemented(&self) -> bool {
        matches!(*self, FileFormat::Fasta | FileFormat::Sam)
    }
}

pub static EXTENSION_MAP: Lazy<HashMap<&'static str, FileFormat>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // FASTA extensions
    map.insert("fasta", FileFormat::Fasta);
    map.insert("fas", FileFormat::Fasta);
    map.insert("fa", FileFormat::Fasta);
    map.insert("fna", FileFormat::Fasta);
    map.insert("ffn", FileFormat::Fasta);
    map.insert("faa", FileFormat::Fasta);
    map.insert("mpfa", FileFormat::Fasta);
    map.insert("frn", FileFormat::Fasta);

    // SAM/BAM
    map.insert("sam", FileFormat::Sam);
    map.insert("bam", FileFormat::Bam);

    // FASTQ
    map.insert("fastq", FileFormat::Fastq);
    map.insert("fq", FileFormat::Fastq);

    // VCF
    map.insert("vcf", FileFormat::Vcf);

    // BED
    map.insert("bed", FileFormat::Bed);

    // GFF/GTF
    map.insert("gff", FileFormat::Gff);
    map.insert("gff3", FileFormat::Gff);
    map.insert("gtf", FileFormat::Gtf);

    map
});

pub fn detect_format_from_path(file_path: &str) -> FileFormat {
    let path = Path::new(file_path);

    if let Some(extension) = path.extension() {
        if let Some(ext_str) = extension.to_str() {
            if let Some(format) = EXTENSION_MAP.get(ext_str.to_lowercase().as_str()) {
                return *format;
            }
        }
    }

    FileFormat::Unknown
}

pub fn has_valid_extension(file_path: &str, format: FileFormat) -> bool {
    let detected = detect_format_from_path(file_path);
    detected == format
}
