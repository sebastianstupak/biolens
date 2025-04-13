pub mod definition;
pub mod handler;

use crate::commands::common::CommandMeta;

pub const META: CommandMeta = CommandMeta {
    name: "validate",
    aliases: &["val"],
    about: "Validate if the file is valid (FASTA, FASTQ, SAM/BAM, VCF, GFF, GTF)",
};
