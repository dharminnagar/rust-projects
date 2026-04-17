// Assumed log format
// [timestamp] [timestamp] [log level] [message]
// 2024-01-15 10:32:01 INFO  Server started on port 8080
// 2024-01-15 10:32:05 WARN  High memory usage detected
// 2024-01-15 10:32:09 ERROR Failed to connect to database

use std::env;
use anyhow::{Result, Context};
use logparse::{log::{LogEntry, LogLevel, read_file}, print_summary, summarize};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let file_path = args.get(1).context("usage: logparse <file> [LEVEL]")?;
    let filter = args.get(2);

    let mut log_entries: Vec<LogEntry> = read_file(file_path).context("Failed to read log file")?;

    log_entries.sort();

    if let Some(level_str) = filter {
        let level = match level_str.as_str() {
            "INFO" => LogLevel::Info,
            "DEBUG" => LogLevel::Debug,
            "WARN" => LogLevel::Warn,
            "ERROR" => LogLevel::Error,
            "UNKNOWN" => LogLevel::Unknown,
            // use error UnknownLevel from ParseError instead of printing to stderr
            _ => anyhow::bail!("Invalid log level filter: {}. Valid options are INFO, DEBUG, WARN, ERROR, UNKNOWN", level_str),
        };
        log_entries.retain(|entry| entry.level == level);
    }

    let summary = summarize(log_entries);
    print_summary(&summary, filter);

    Ok(())
}
