use clap::{Arg, ArgAction};

// Common CLI argument for visualization
pub fn viz_arg() -> Arg {
    Arg::new("visualize")
        .short('v')
        .long("visualize")
        .action(ArgAction::SetTrue)
        .help("Generate visualization")
}

// Command metadata structure
pub struct CommandMeta {
    pub name: &'static str,
    pub aliases: &'static [&'static str],
    pub about: &'static str,
}

// Command registration information
pub struct CommandInfo {
    pub name: &'static str,
    pub aliases: Vec<&'static str>,
    pub handler: fn(&clap::ArgMatches), // Function pointer to command handler
}

// Visualization wrapper function
#[cfg(feature = "visualization")]
pub fn with_visualization<F, T>(f: F, result: &T)
where
    F: FnOnce(&T),
{
    f(result);
}

#[cfg(not(feature = "visualization"))]
pub fn with_visualization<F, T>(_f: F, _result: &T)
where
    F: FnOnce(&T),
{
    eprintln!("Visualization feature not enabled. Recompile with --features visualization");
}
