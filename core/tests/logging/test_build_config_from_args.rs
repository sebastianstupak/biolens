use biolens_core::logging;

#[test]
fn test_build_config_from_args() {
    let config = logging::build_config_from_args(2, true, true);
    assert_eq!(config.verbosity, 2);
    assert!(config.log_to_file);
    assert!(config.json_format);
    assert_eq!(config.max_log_files, Some(5)); // Default value preserved
}
