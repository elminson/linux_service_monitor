use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::process::Command;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use serde_json::json;
use chrono::prelude::*;

#[derive(Deserialize)]
struct ServiceConfig {
    name: String,
    active_text: String,
    inactive_text: String,
}

#[derive(Deserialize)]
struct Config {
    debug: bool,
    sleep: u64,
    services: Vec<ServiceConfig>,
}

#[derive(Serialize)]
struct LogEntry {
    service: String,
    status: String,
    timestamp: u64,
    datetime: String,
}

fn read_config(file_path: &str) -> Result<Config, io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: Config = serde_json::from_str(&contents)?;
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

fn log_error_to_file(error: &str, datetime_str: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("error.log")
        .expect("Failed to open error log file");

    let start_time = SystemTime::now();
    let elapsed = start_time.duration_since(UNIX_EPOCH).expect("SystemTime before UNIX EPOCH");
    let timestamp = elapsed.as_secs();

    writeln!(file, "[{} | {}] {}", datetime_str, timestamp, error).expect("Failed to write to error log file");
}

fn main() {
    let config = read_config("config.json").expect("Failed to read config file");

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
                    log_error_to_file(&err, &datetime_str);
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