# BioLens Advanced Logging System Guide

This document explains how to use the advanced logging system implemented in the BioLens application.

## Features

The logging system includes the following features:

1. **Multiple Verbosity Levels** - Control the amount of log information displayed
2. **File Logging** - Save logs to rotating files for later analysis
3. **Structured Logging** - Use the `tracing` crate for better log organization and analysis
4. **Configuration Files** - Customize logging through configuration files

## Command Line Options

The following command line options control logging:

| Option                | Description                                     |
| --------------------- | ----------------------------------------------- |
| `-v`, `-vv`, `-vvv`   | Increase verbosity (can be used multiple times) |
| `--log-file`          | Write logs to file (in addition to console)     |
| `--json-logs`         | Output logs in JSON format (machine-readable)   |
| `-c, --config <FILE>` | Path to custom configuration file               |

## Examples

### Basic Usage

```bash
# Normal output (INFO level)
biolens validate -i sample.fasta

# With debug output (DEBUG level)
biolens -v validate -i sample.fasta

# With trace output (TRACE level)
biolens -vv validate -i sample.fasta

# Extremely verbose output (maximum tracing)
biolens -vvv validate -i sample.fasta
```

### File Logging

```bash
# Log to file with default settings
biolens --log-file validate -i sample.fasta

# Log to file with JSON format (good for log analyzers)
biolens --log-file --json-logs validate -i sample.fasta

# Verbose logging to file
biolens -vv --log-file validate -i sample.fasta
```

## Configuration File

The configuration file is automatically created at the following locations:

- **Linux**: `~/.config/biolens/logging.toml`
- **macOS**: `~/Library/Application Support/com.biolens.biolens/logging.toml`
- **Windows**: `%APPDATA%\biolens\logging.toml`

You can customize the following settings:

```toml
# BioLens Logging Configuration File

# Verbosity level (0 = INFO, 1 = DEBUG, 2 = TRACE, 3+ = MAX TRACE)
verbosity = 0

# Path to log directory (leave empty for default location)
log_directory = ""

# Whether to log to a file
log_to_file = false

# Whether to use JSON format for logs (useful for machine parsing)
json_format = false

# Maximum number of log files to keep before rotation
max_log_files = 5

# Maximum log file size in MB before rotation
max_log_size_mb = 10
```

Command line arguments will override settings in the configuration file.

## Log File Management

Log files are automatically managed:

1. Files are rotated when they reach the configured maximum size
2. Old log files are automatically pruned when they exceed the maximum number
3. Log files follow the naming convention: `biolens.{timestamp}.log`

Log files are stored in:

- **Linux**: `~/.local/share/biolens/logs/`
- **macOS**: `~/Library/Application Support/com.biolens.biolens/logs/`
- **Windows**: `%APPDATA%\biolens\logs\`

## For Developers: Using the Logging System

When writing code for BioLens, use the following macros from the `tracing` crate:

```rust
use tracing::{trace, debug, info, warn, error, instrument};

// Different log levels
fn example_function() {
    // These levels will only be displayed with increasing verbosity:
    error!("Critical error - always shown");         // Always shown
    warn!("Warning - always shown");                 // Always shown
    info!("Information - shown by default");         // Shown by default
    debug!("Debug details - shown with -v");         // Only with -v
    trace!("Trace details - shown with -vv");        // Only with -vv

    // With structured fields (great for JSON logs)
    info!(
        file_size = 1024,
        file_name = "example.fasta",
        "Processing file"
    );

    // With span for grouping related log messages
    #[instrument(name = "processing_file", fields(file = "example.fasta"))]
    fn process_file() {
        // All logs in this function will be grouped
        // under the "processing_file" span
        debug!("Started processing");
        // ... do work ...
        info!("Finished processing");
    }
}
```

### Best Practices

1. **Use Appropriate Levels**:

   - `error!` - Fatal or critical issues that prevent operation
   - `warn!` - Problems that don't stop execution but need attention
   - `info!` - Regular operational messages (default level)
   - `debug!` - Detailed information for developers
   - `trace!` - Ultra-detailed debugging information

2. **Add Context with Fields**:

   ```rust
   info!(
       file_path = %path,
       file_size = size,
       "File processed successfully"
   );
   ```

3. **Use Spans for Operations**:

   ```rust
   #[instrument(skip(large_input), fields(filename = %path))]
   fn process_file(path: &str, large_input: &[u8]) {
       // ...
   }
   ```

4. **Handle Expensive Logging Efficiently**:
   ```rust
   if tracing::enabled!(tracing::Level::TRACE) {
       // Only do expensive computation if TRACE is enabled
       let result = expensive_calculation();
       trace!(result = ?result, "Calculation result");
   }
   ```

## Performance Considerations

- JSON logging is more CPU-intensive but better for automated analysis
- File logging adds some I/O overhead
- The non-blocking logger minimizes performance impact
- At the highest verbosity levels, logging can impact performance

## Analyzing Logs

When logging in JSON format, you can use tools like `jq` to analyze the logs:

```bash
# Find all error messages
cat biolens.log | jq 'select(.level == "ERROR")'

# Count logs by level
cat biolens.log | jq '.level' | sort | uniq -c

# Find logs for a specific file
cat biolens.log | jq 'select(.fields.file_path == "example.fasta")'
```
