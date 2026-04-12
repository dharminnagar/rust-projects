// Assumed log format
// [timestamp] [timestamp] [log level] [message]
// 2024-01-15 10:32:01 INFO  Server started on port 8080
// 2024-01-15 10:32:05 WARN  High memory usage detected
// 2024-01-15 10:32:09 ERROR Failed to connect to database

use std::env;
use std::fmt::Display;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
// use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Clone, PartialOrd, Ord)]
enum LogLevel {
    Info,
    Debug,
    Warn,
    Error,
    Unknown
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, PartialOrd, Ord)]
struct LogEntry {
    timestamp: String,
    level: LogLevel,
    message: String,
}

fn read_file(path: &str) -> Vec<LogEntry> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => panic!("File not Found"),
    };

    let mut reader = BufReader::new(file);
    
    let mut contents = String::new();
    
    reader.read_to_string(&mut contents).expect("Failed to read file");

    contents.lines().filter_map(parse_line).collect()
}

fn parse_line(line: &str) -> Option<LogEntry> {
    let parts: Vec<&str> = line.split(' ').collect();

    let timestamp = parts.first()?.to_string() + " " + parts.get(1)?;
    let log_level = match *parts.get(2)? {
        "INFO" => LogLevel::Info,
        "DEBUG" => LogLevel::Debug,
        "WARN" => LogLevel::Warn,
        "ERROR" => LogLevel::Error,
        _ => LogLevel::Unknown,
    };
    let message = parts[3..].join(" ");

    Some(LogEntry {
        timestamp,
        level: log_level,
        message,
    })
}

// Summary example
// ERROR   12 entries
// WARN     8 entries
// INFO    43 entries
fn summarize(entries: Vec<LogEntry>) -> HashMap<LogLevel, Vec<LogEntry>> {
    let mut summary: HashMap<LogLevel, Vec<LogEntry>> = HashMap::new();

    for entry in entries {
        summary.entry(entry.level.clone()).or_default().push(entry);
    }
    
    summary
}

fn print_summary(map: &HashMap<LogLevel, Vec<LogEntry>>, filter: Option<&String>) {
    let filter = match filter {
        Some(filter) => filter,
        None => "All Levels"
    };

    println!("========================");
    println!("Summary");
    println!("========================");

    for (level, entries) in map {
        println!("{:?} => {} entries", level, entries.len());
    }
    
    println!();
    println!("========================");
    println!("Entries (level: {})", filter);
    println!("========================");

    for entries in map.values() {
        // println!("Level: {:?} => {} entries", level, entries.len());
        for entry in entries {
            if filter == "All Levels" {
                println!("{} [{}] - {}", entry.timestamp, entry.level, entry.message);
            } else {
                println!("{} - {}", entry.timestamp, entry.message);
            }
        }
    }

    println!();
    println!("========================");
    println!("End of Summary");
    println!("========================");

}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = args.get(1).expect("usage: logparse <file> [LEVEL]");
    let filter = args.get(2);

    let mut log_entries: Vec<LogEntry> = read_file(file_path);

    log_entries.sort();

    if let Some(level_str) = filter {
        let level = match level_str.as_str() {
            "INFO" => LogLevel::Info,
            "DEBUG" => LogLevel::Debug,
            "WARN" => LogLevel::Warn,
            "ERROR" => LogLevel::Error,
            _ => LogLevel::Unknown,
        };
        log_entries.retain(|entry| entry.level == level);
    }

    let summary = summarize(log_entries);
    print_summary(&summary, filter);
}
