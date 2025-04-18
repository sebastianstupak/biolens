use directories::ProjectDirs;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tracing::{debug, info, Level};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt, EnvFilter};

// We use a Lazy static to ensure we keep the guard alive for the duration of the program
static WRITER_GUARD: Lazy<Mutex<Option<WorkerGuard>>> = Lazy::new(|| Mutex::new(None));

// Default config path locations
fn get_config_dir() -> Option<PathBuf> {
    ProjectDirs::from("com", "biolens", "biolens").map(|dirs| dirs.config_dir().to_path_buf())
}

fn get_default_log_dir() -> Option<PathBuf> {
    ProjectDirs::from("com", "biolens", "biolens").map(|dirs| dirs.data_dir().join("logs"))
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LogConfig {
    /// Verbosity level (0-3)
    pub verbosity: u8,
    /// Path to log directory, relative paths are resolved relative to the executable
    pub log_directory: Option<String>,
    /// Whether to log to a file
    pub log_to_file: bool,
    /// Whether to use JSON format (for machine parsing)
    pub json_format: bool,
    /// Maximum number of log files to keep
    pub max_log_files: Option<u32>,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            verbosity: 0,
            log_directory: None,
            log_to_file: false,
            json_format: false,
            max_log_files: Some(5),
        }
    }
}

impl LogConfig {
    pub fn load_from_file(path: &Path) -> Result<Self, String> {
        let mut file =
            File::open(path).map_err(|e| format!("Failed to open config file: {}", e))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| format!("Failed to read config file: {}", e))?;

        toml::from_str(&contents).map_err(|e| format!("Failed to parse config: {}", e))
    }

    pub fn save_to_file(&self, path: &Path) -> Result<(), String> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;

        // Ensure the directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
        }

        fs::write(path, content).map_err(|e| format!("Failed to write config: {}", e))
    }

    pub fn get_log_level(&self) -> Level {
        match self.verbosity {
            0 => Level::INFO,
            1 => Level::DEBUG,
            2 => Level::TRACE,
            _ => Level::TRACE,
        }
    }

    pub fn get_log_directory(&self) -> PathBuf {
        if let Some(dir) = &self.log_directory {
            PathBuf::from(dir)
        } else if let Some(data_dir) = get_default_log_dir() {
            data_dir
        } else {
            PathBuf::from("./logs")
        }
    }
}

/// Create or load a default config file
pub fn ensure_config_file() -> Result<PathBuf, String> {
    if let Some(config_dir) = get_config_dir() {
        let config_path = config_dir.join("logging.toml");

        if !config_path.exists() {
            fs::create_dir_all(&config_dir)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;

            let default_config = LogConfig::default();
            default_config.save_to_file(&config_path)?;

            eprintln!("Created default logging config at {:?}", config_path);
        }

        Ok(config_path)
    } else {
        Err("Could not determine config directory".to_string())
    }
}

/// Build a LogConfig from command line arguments
pub fn build_config_from_args(verbosity: u8, log_to_file: bool, json_format: bool) -> LogConfig {
    LogConfig {
        verbosity,
        log_to_file,
        json_format,
        ..LogConfig::default()
    }
}

/// Load config from file, but override with command line arguments
pub fn load_config(
    verbosity_arg: Option<u8>,
    log_to_file_arg: Option<bool>,
    json_format_arg: Option<bool>,
) -> LogConfig {
    let config_result = match ensure_config_file() {
        Ok(path) => LogConfig::load_from_file(&path),
        Err(e) => {
            eprintln!("Warning: {}", e);
            Ok(LogConfig::default())
        }
    };

    let mut config = config_result.unwrap_or_else(|e| {
        eprintln!("Warning: Failed to load config: {}", e);
        LogConfig::default()
    });

    // Override with command line arguments if provided
    if let Some(v) = verbosity_arg {
        config.verbosity = v;
    }

    if let Some(l) = log_to_file_arg {
        config.log_to_file = l;
    }

    if let Some(j) = json_format_arg {
        config.json_format = j;
    }

    config
}

/// Initialize the logging system based on the provided configuration
pub fn setup_logger(config: LogConfig) -> Result<(), String> {
    // Create the filter based on verbosity
    let filter = EnvFilter::from_default_env().add_directive(config.get_log_level().into());

    // Handle JSON vs regular format separately to avoid type mismatches
    if config.json_format {
        setup_json_logging(config, filter)
    } else {
        setup_text_logging(config, filter)
    }
}

/// Set up JSON-formatted logging
fn setup_json_logging(config: LogConfig, filter: EnvFilter) -> Result<(), String> {
    if config.log_to_file {
        // Set up file logging
        let log_dir = config.get_log_directory();

        // Create log directory
        fs::create_dir_all(&log_dir)
            .map_err(|e| format!("Failed to create log directory: {}", e))?;

        // Create file appender
        let appender = RollingFileAppender::builder()
            .rotation(Rotation::DAILY)
            .filename_prefix("biolens")
            .max_log_files(config.max_log_files.unwrap_or(5) as usize)
            .build(log_dir)
            .map_err(|e| format!("Failed to create log file: {}", e))?;

        // Create non-blocking writer
        let (non_blocking, guard) = tracing_appender::non_blocking(appender);

        // Store guard to keep worker thread alive
        let mut global_guard = WRITER_GUARD
            .lock()
            .map_err(|_| "Failed to acquire lock on writer guard".to_string())?;
        *global_guard = Some(guard);

        // Set up subscriber with file and console output
        let subscriber = fmt::Subscriber::builder()
            .with_env_filter(filter)
            .with_ansi(false) // No ANSI colors in file
            .with_writer(non_blocking)
            .json()
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .map_err(|e| format!("Failed to set global subscriber: {}", e))?;
    } else {
        // JSON output to console only
        let subscriber = fmt::Subscriber::builder()
            .with_env_filter(filter)
            .with_ansi(true)
            .json()
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .map_err(|e| format!("Failed to set global subscriber: {}", e))?;
    }

    // Log initialization
    debug!(
        "Logger initialized with verbosity level {} (JSON format)",
        config.verbosity
    );

    if config.log_to_file {
        info!("Logging to directory: {:?}", config.get_log_directory());
    }

    Ok(())
}

/// Set up plain text logging
fn setup_text_logging(config: LogConfig, filter: EnvFilter) -> Result<(), String> {
    if config.log_to_file {
        // Set up file logging
        let log_dir = config.get_log_directory();

        // Create log directory
        fs::create_dir_all(&log_dir)
            .map_err(|e| format!("Failed to create log directory: {}", e))?;

        // Create file appender
        let appender = RollingFileAppender::builder()
            .rotation(Rotation::DAILY)
            .filename_prefix("biolens")
            .max_log_files(config.max_log_files.unwrap_or(5) as usize)
            .build(log_dir)
            .map_err(|e| format!("Failed to create log file: {}", e))?;

        // Create non-blocking writer
        let (non_blocking, guard) = tracing_appender::non_blocking(appender);

        // Store guard to keep worker thread alive
        let mut global_guard = WRITER_GUARD
            .lock()
            .map_err(|_| "Failed to acquire lock on writer guard".to_string())?;
        *global_guard = Some(guard);

        // Create a subscriber for file output
        let file_subscriber = fmt::Subscriber::builder()
            .with_env_filter(filter)
            .with_ansi(false) // No ANSI colors in file
            .with_writer(non_blocking)
            .finish();

        tracing::subscriber::set_global_default(file_subscriber)
            .map_err(|e| format!("Failed to set global subscriber: {}", e))?;
    } else {
        // Plain text output to console only
        let subscriber = fmt::Subscriber::builder()
            .with_env_filter(filter)
            .with_ansi(true) // Use colors in terminal
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .map_err(|e| format!("Failed to set global subscriber: {}", e))?;
    }

    // Log initialization
    debug!(
        "Logger initialized with verbosity level {}",
        config.verbosity
    );

    if config.log_to_file {
        info!("Logging to directory: {:?}", config.get_log_directory());
    }

    Ok(())
}
