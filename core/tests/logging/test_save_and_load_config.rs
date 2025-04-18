use biolens_core::logging::LogConfig;
use tempfile::tempdir;

#[test]
fn test_save_and_load_config() {
    // Create a temporary directory for the test
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let config_path = temp_dir.path().join("logging.toml");

    // Create a test config
    let config = LogConfig {
        verbosity: 2,
        log_directory: Some("test_logs".to_string()),
        log_to_file: true,
        json_format: true,
        max_log_files: Some(10),
    };

    // Save the config
    config
        .save_to_file(&config_path)
        .expect("Failed to save config");

    // Verify file exists
    assert!(config_path.exists());

    // Load the config back
    let loaded_config = LogConfig::load_from_file(&config_path).expect("Failed to load config");

    // Verify it's the same
    assert_eq!(loaded_config.verbosity, config.verbosity);
    assert_eq!(loaded_config.log_directory, config.log_directory);
    assert_eq!(loaded_config.log_to_file, config.log_to_file);
    assert_eq!(loaded_config.json_format, config.json_format);
    assert_eq!(loaded_config.max_log_files, config.max_log_files);
}
