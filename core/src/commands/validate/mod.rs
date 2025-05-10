pub mod definition;
pub mod handler;

pub mod common;
pub mod fasta;
pub mod formats;
pub mod sam;

pub struct ModuleMetadata {
    pub name: &'static str,
    pub aliases: &'static [&'static str],
    pub about: &'static str,
}

pub const META: ModuleMetadata = ModuleMetadata {
    name: "validate",
    aliases: &["val", "check"],
    about: "Validate bioinformatics file formats",
};
