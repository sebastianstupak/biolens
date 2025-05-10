use std::io::{self, IsTerminal};

/// Trait that must be implemented by all validation result types
pub trait ValidationResultTrait {
    /// Returns whether the validation was successful
    fn is_valid(&self) -> bool;

    /// Print a formatted summary of validation results to stdout
    fn print_summary(&self);

    /// Print a minimal result for non-interactive/piped output
    fn print_minimal(&self) {
        // Default implementation just prints "valid" or "invalid"
        if self.is_valid() {
            println!("valid");
        } else {
            println!("invalid");
        }
    }
}

/// Trait that must be implemented by all validators
pub trait Validator {
    type Options;
    type Result: ValidationResultTrait;

    /// Build options from command-line arguments
    fn build_options_from_args(matches: &clap::ArgMatches) -> Self::Options;

    /// Validate a file with the given options
    fn validate(file_path: &str, options: Self::Options) -> Self::Result;

    /// Handle printing the validation results appropriately based on output context
    fn print_results(result: &Self::Result) -> bool {
        if io::stdout().is_terminal() {
            // Interactive mode - print full summary
            result.print_summary();
        } else {
            // Piped output - print minimal result
            result.print_minimal();
        }

        result.is_valid()
    }
}
