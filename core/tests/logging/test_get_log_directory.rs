use biolens_core::logging::LogConfig;
use std::path::Path;

#[test]
fn test_get_log_directory() {
    // Default directory
    let config = LogConfig::default();
    let default_dir = config.get_log_directory();

    // Custom directory
    let config = LogConfig {
        log_directory: Some("/custom/path".to_string()),
        ..Default::default()
    };
    let custom_dir = config.get_log_directory();

    assert_ne!(default_dir, custom_dir);
    assert_eq!(custom_dir, Path::new("/custom/path"));
}
