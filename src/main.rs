mod tests;

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::process::{Command, exit};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use serde_json::json;
use chrono::prelude::*;

#[derive(Deserialize, Serialize)]
struct ServiceConfig {
    name: String,
    active_text: String,
    inactive_text: String,
}

#[derive(Deserialize, Serialize)]
struct Config {
    debug: bool,
    sleep: u64,
    services: Vec<ServiceConfig>,
    log_file: String,
}

#[derive(Serialize)]
struct LogEntry {
    service: String,
    status: String,
    timestamp: u64,
    datetime: String,
}

fn read_config(file_path: &str) -> Result<Config, io::Error> {
    if !std::path::Path::new(file_path).exists() {
        let default_config = Config {
            debug: true,
            sleep: 2,
            services: vec![
                ServiceConfig {
                    name: "fake_service".to_string(),
                    active_text: "online".to_string(),
                    inactive_text: "down".to_string(),
                }
            ],
            log_file: "error.log".to_string(), // Set default log file
        };
        let default_config_json = serde_json::to_string_pretty(&default_config).expect("Failed to serialize default config");
        let mut file = File::create(file_path)?;
        file.write_all(default_config_json.as_bytes())?;
    }

    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut config: Config = serde_json::from_str(&contents)?;

    // Set default log file if not present
    if config.log_file.is_empty() {
        config.log_file = "error.log".to_string();
    }

    Ok(config)
}

fn check_service_status(service: &ServiceConfig, debug: bool) -> Result<String, String> {
    let output = Command::new("service")
        .arg(&service.name)
        .arg("status")
        .output();
    if debug {
        println!("Checking status of {}", service.name);
        println!("{:?}", output);
    }
    match output {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if stdout.contains(&service.active_text) {
                    Ok("active".to_string())
                } else if stdout.contains(&service.inactive_text) {
                    Ok("inactive".to_string())
                } else {
                    Ok("unknown".to_string())
                }
            } else {
                io::stderr().write_all(&output.stderr).unwrap();
                Err(format!("Failed to check status of {}", service.name))
            }
        },
        Err(_) => Err(format!("Failed to execute service command for {}", service.name)),
    }
}

fn log_error_to_file(error: &str, datetime_str: &str, log_file: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file)
        .expect("Failed to open error log file");

    let start_time = SystemTime::now();
    let elapsed = start_time.duration_since(UNIX_EPOCH).expect("SystemTime before UNIX EPOCH");
    let timestamp = elapsed.as_secs();

    writeln!(file, "[{} | {}] {}", datetime_str, timestamp, error).expect("Failed to write to error log file");
}

fn main() {
    let config = match read_config("config.json") {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Failed to read config file: {}", err);
            exit(1);
        }
    };

    loop {
        let start_time = SystemTime::now();

        let datetime: DateTime<Utc> = Utc::now();
        let datetime_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

        let log_entries: Vec<LogEntry> = config.services.iter().map(|service| {
            let status = match check_service_status(service, config.debug) {
                Ok(status) => status,
                Err(err) => {
                    if config.debug {
                        eprintln!("Error: {}", err);
                    }
                    log_error_to_file(&err, &datetime_str, &config.log_file);
                    "error".to_string()
                }
            };

            let elapsed = start_time.duration_since(UNIX_EPOCH).expect("SystemTime before UNIX EPOCH");
            LogEntry {
                service: service.name.clone(),
                status,
                timestamp: elapsed.as_secs(),
                datetime: datetime_str.clone(),
            }
        }).collect();

        let log_json = json!(log_entries);
        println!("{}", serde_json::to_string_pretty(&log_json).expect("Failed to serialize JSON"));

        thread::sleep(Duration::from_secs(config.sleep));
    }
}