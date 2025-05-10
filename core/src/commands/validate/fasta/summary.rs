use super::result::{IssueLevel, ValidationResult};

pub fn print_summary(result: &ValidationResult) {
    if result.is_valid {
        println!("FASTA file is valid");
    } else {
        println!("FASTA file is invalid");
    }

    println!("Sequences: {}", result.sequence_count);

    if result.sequence_count > 0 {
        println!("Total sequence length: {}", result.total_sequence_length);
        println!(
            "Shortest sequence: {} characters",
            result.min_observed_sequence_length
        );
        println!(
            "Longest sequence: {} characters",
            result.max_observed_sequence_length
        );

        if let Some(seq_type) = result.detected_sequence_type {
            println!("Detected sequence type: {:?}", seq_type);
        }
    }

    let errors = result
        .issues
        .iter()
        .filter(|i| i.level == IssueLevel::Error)
        .count();
    let warnings = result
        .issues
        .iter()
        .filter(|i| i.level == IssueLevel::Warning)
        .count();
    let infos = result
        .issues
        .iter()
        .filter(|i| i.level == IssueLevel::Info)
        .count();

    if errors > 0 || warnings > 0 || infos > 0 {
        println!("\nValidation Issues:");
        println!("  Errors: {}", errors);
        println!("  Warnings: {}", warnings);
        println!("  Infos: {}", infos);

        for issue in &result.issues {
            let prefix = match issue.level {
                IssueLevel::Error => "ERROR",
                IssueLevel::Warning => "WARNING",
                IssueLevel::Info => "INFO",
            };

            let location = if let Some(line) = issue.line_number {
                format!("Line {}", line)
            } else if let Some(header) = &issue.header {
                format!("In sequence '{}'", header)
            } else {
                "".to_string()
            };

            println!("{}: {} {}", prefix, issue.message, location);
        }
    }
}
