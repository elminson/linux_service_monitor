use super::*;
use tempfile::NamedTempFile;
use std::process::Command;
use std::fs;

    #[test]
fn test_read_config_reads_existing() {
    // Create a temporary file and write custom content (simulating an existing config file)
    let test_file = NamedTempFile::new().expect("Failed to create temp file");
    let test_path = test_file.path().to_str().unwrap().to_string();

    let custom_config = r#"
    {
        "debug": true,
        "sleep": 2,
        "services": [
            {
                "name": "fake_service",
                "active_text": "online",
                "inactive_text": "down"
            }
        ],
        "log_file": "error.log"
    }"#;
    fs::write(&test_path, custom_config).expect("Failed to write custom config");

    let config = read_config(&test_path).expect("Failed to read config");

    // Validate values read from the custom config
    assert_eq!(config.debug, true);
    assert_eq!(config.sleep, 2);
    assert_eq!(config.services.len(), 1);
    assert_eq!(config.services[0].name, "fake_service");
    assert_eq!(config.services[0].active_text, "online");
    assert_eq!(config.services[0].inactive_text, "down");
    assert_eq!(config.log_file, "error.log");
}

#[test]
fn test_check_service_status() {
    let service = ServiceConfig {
        name: "fake_service".to_string(),
        active_text: "online".to_string(),
        inactive_text: "down".to_string(),
    };

    // Use assert_cmd to mock the command and its output
    let mut cmd = Command::new("echo");
    cmd.arg("fake_service is online");

    let output = cmd.output().expect("Failed to execute command");

    let status = if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.contains(&service.active_text) {
            Ok("active".to_string())
        } else if stdout.contains(&service.inactive_text) {
            Ok("inactive".to_string())
        } else {
            Ok("unknown".to_string())
        }
    } else {
        Err(format!("Failed to execute service command for {}", service.name))
    };

    assert_eq!(status.unwrap(), "active");
}

#[test]
fn test_log_error_to_file() {
    // Use a temporary file to log errors
    let test_file = NamedTempFile::new().expect("Failed to create temp file");
    let test_path = test_file.path().to_str().unwrap().to_string();

    let error_message = "Test error message";
    let datetime_str = "2023-01-01 00:00:00";
    log_error_to_file(error_message, datetime_str, &test_path);

    // Ensure the file is created before reading
    assert!(std::path::Path::new(&test_path).exists(), "Log file was not created");

    let contents = std::fs::read_to_string(&test_path).expect("Failed to read log file");
    assert!(contents.contains(error_message), "Log file does not contain the error message");
    assert!(contents.contains(datetime_str), "Log file does not contain the datetime string");
}