pub mod definition;
pub mod handler;

use crate::commands::common::CommandMeta;

pub const META: CommandMeta = CommandMeta {
    name: "bam-coverage",
    aliases: &["bamcov", "bam-cov"],
    about: "Calculate coverage from BAM files",
};
