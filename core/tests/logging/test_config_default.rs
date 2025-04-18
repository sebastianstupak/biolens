use biolens_core::logging::LogConfig;

#[test]
fn test_config_default() {
    let config = LogConfig::default();
    assert_eq!(config.verbosity, 0);
    assert!(!config.log_to_file);
    assert!(!config.json_format);
    assert_eq!(config.max_log_files, Some(5));
}
