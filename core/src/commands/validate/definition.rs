use super::super::common::CommandInfo;
use super::{handler, META};
use clap::{Arg, ArgAction, ArgMatches, Command};

pub fn command_info() -> CommandInfo {
    CommandInfo {
        name: META.name,
        aliases: META.aliases.to_vec(),
        handler: handle,
    }
}

pub fn command() -> Command {
    Command::new(META.name)
        .aliases(META.aliases)
        .about(META.about)
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .required(true)
                .help("Input file"),
        )
        // General validation options
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .help("Only output validation result (success/fail)")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("format")
                .short('f')
                .long("format")
                .help("Force specific file format (override auto-detection)")
                .value_name("FORMAT")
                .value_parser([
                    "fasta", "fas", "fa", "fna", "ffn", "faa", "mpfa", "frn", "sam", "bam",
                    "fastq", "fq", "vcf", "bed", "gff", "gtf",
                ]),
        )
        .group(clap::ArgGroup::new("fasta-options").multiple(true).args([
            "sequence-type",
            "no-line-length-check",
            "max-line-length",
            "min-sequence-length",
            "max-sequence-length",
            "disallow-lowercase",
            "keep-asterisk",
            "standardize",
            "validate-ncbi",
            "max-header-length",
            "no-extension-check",
            "disallow-empty-lines",
            "disallow-comments",
        ]))
        // FASTA-specific arguments
        .arg(
            Arg::new("sequence-type")
                .long("sequence-type")
                .value_name("TYPE")
                .help("FASTA: Specify sequence type (auto, nucleotide, protein)")
                .value_parser(["auto", "nucleotide", "protein"])
                .default_value("auto"),
        )
        .arg(
            Arg::new("no-line-length-check")
                .long("no-line-length-check")
                .help("FASTA: Disable checking sequence line length")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("max-line-length")
                .long("max-line-length")
                .value_name("LENGTH")
                .help("FASTA: Maximum allowed sequence line length")
                .value_parser(clap::value_parser!(usize))
                .default_value("80"),
        )
        .arg(
            Arg::new("min-sequence-length")
                .long("min-sequence-length")
                .value_name("LENGTH")
                .help("FASTA: Minimum allowed sequence length")
                .value_parser(clap::value_parser!(usize))
                .default_value("1"),
        )
        .arg(
            Arg::new("max-sequence-length")
                .long("max-sequence-length")
                .value_name("LENGTH")
                .help("FASTA: Maximum allowed sequence length")
                .value_parser(clap::value_parser!(usize)),
        )
        .arg(
            Arg::new("disallow-lowercase")
                .long("disallow-lowercase")
                .help("FASTA: Disallow lowercase letters in sequences")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("keep-asterisk")
                .long("keep-asterisk")
                .help("FASTA: Don't ignore asterisk at end of sequence")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("standardize")
                .long("standardize")
                .help("FASTA: Standardize sequences (convert to uppercase)")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("validate-ncbi")
                .long("validate-ncbi")
                .help("FASTA: Validate NCBI header format")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("max-header-length")
                .long("max-header-length")
                .value_name("LENGTH")
                .help("FASTA: Maximum allowed header length")
                .value_parser(clap::value_parser!(usize)),
        )
        .arg(
            Arg::new("no-extension-check")
                .long("no-extension-check")
                .help("FASTA: Disable file extension checking")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("disallow-empty-lines")
                .long("disallow-empty-lines")
                .help("FASTA: Disallow empty lines in file")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("disallow-comments")
                .long("disallow-comments")
                .help("FASTA: Disallow comment lines (starting with ';')")
                .action(ArgAction::SetTrue),
        )
    // SAM-specific options group (placeholder for now)
    // .group(
    //     clap::ArgGroup::new("sam-options")
    //         .multiple(true)
    //         .args([])
    // )
}

pub fn handle(matches: &ArgMatches) {
    let input_file = matches.get_one::<String>("input").unwrap();
    let quiet_mode = matches.get_flag("quiet");

    let format_override = matches.get_one::<String>("format").map(|s| s.as_str());

    let result = handler::validate(input_file, matches, format_override);

    // If we're in quiet mode, just exit with appropriate code
    if quiet_mode {
        std::process::exit(if result { 0 } else { 1 });
    }
}
