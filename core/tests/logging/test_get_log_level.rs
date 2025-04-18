use biolens_core::logging::LogConfig;
use tracing::Level;

#[test]
fn test_get_log_level() {
    // Test INFO level
    let config = LogConfig {
        verbosity: 0,
        ..Default::default()
    };
    assert_eq!(config.get_log_level(), Level::INFO);

    // Test DEBUG level
    let config = LogConfig {
        verbosity: 1,
        ..Default::default()
    };
    assert_eq!(config.get_log_level(), Level::DEBUG);

    // Test TRACE level (verbosity 2)
    let config = LogConfig {
        verbosity: 2,
        ..Default::default()
    };
    assert_eq!(config.get_log_level(), Level::TRACE);

    // Test TRACE level (verbosity 3)
    let config = LogConfig {
        verbosity: 3,
        ..Default::default()
    };
    assert_eq!(config.get_log_level(), Level::TRACE);
}
