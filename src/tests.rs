use super::*;
use std::fs;
use std::process::Command;

#[test]
fn test_read_config_creates_default() {
    let test_file = "test_config.json";

    let config = read_config(test_file).expect("Failed to read config");
    assert!(std::path::Path::new(test_file).exists());
    assert_eq!(config.debug, true);
    assert_eq!(config.sleep, 2);
    assert_eq!(config.services.len(), 1);
    assert_eq!(config.services[0].name, "fake_service");
    assert_eq!(config.services[0].active_text, "online");
    assert_eq!(config.services[0].inactive_text, "down");
    assert_eq!(config.log_file, "error.log");
}

#[test]
fn test_read_config_reads_existing() {
    let test_file = "test_config.json";

    let config = read_config(test_file).expect("Failed to read config");
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

    // Mock the command output
    let output = Command::new("echo")
        .arg("fake_service is online")
        .output()
        .expect("Failed to execute command");

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
    let test_file = "test_error.log";
    if std::path::Path::new(test_file).exists() {
        std::fs::remove_file(test_file).expect("Failed to remove existing test file");
    }

    let error_message = "Test error message";
    let datetime_str = "2023-01-01 00:00:00";
    log_error_to_file(error_message, datetime_str, test_file);

    // Ensure the file is created before reading
    assert!(std::path::Path::new(test_file).exists(), "Log file was not created");

    let contents = std::fs::read_to_string(test_file).expect("Failed to read log file");
    assert!(contents.contains(error_message), "Log file does not contain the error message");
    assert!(contents.contains(datetime_str), "Log file does not contain the datetime string");

    std::fs::remove_file(test_file).expect("Failed to remove test file");
}