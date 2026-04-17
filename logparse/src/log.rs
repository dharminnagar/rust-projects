use std::fmt::Display;
use thiserror::Error;
use std::io::{BufReader, Read};
use std::fs::File;

#[derive(Debug, Eq, PartialEq, Hash, Clone, PartialOrd, Ord)]
pub enum LogLevel {
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
pub struct LogEntry {
    pub timestamp: String,
    pub level: LogLevel,
    pub message: String,
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("invalid log format: {line}")]
    InvalidFormat { line: String },

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("unknown log level: {0}")]
    UnknownLevel(String),
}


pub fn parse_line(line: &str) -> Result<LogEntry, ParseError> {

    let parts: Vec<&str> = line.split(' ').collect();

    let timestamp = parts.first()
        .ok_or_else(|| ParseError::InvalidFormat { line: line.to_string() })?.to_string()
        
        + " " 
        
        + parts.get(1)
            .ok_or_else(|| ParseError::InvalidFormat { line: line.to_string() })?;

    let log_level = match *parts.get(2).ok_or_else(|| ParseError::InvalidFormat { line: line.to_string() })? {
        "INFO" => LogLevel::Info,
        "DEBUG" => LogLevel::Debug,
        "WARN" => LogLevel::Warn,
        "ERROR" => LogLevel::Error,
        _ => LogLevel::Unknown,
    };
    let message = parts[3..].join(" ");

    Ok(LogEntry {
        timestamp,
        level: log_level,
        message,
    })
}


pub fn read_file(path: &str) -> Result<Vec<LogEntry>, ParseError> {
    let file = File::open(path)?;

    let mut reader = BufReader::new(file);
    
    let mut contents = String::new();
    
    reader.read_to_string(&mut contents)?;

    let mut log_entries: Vec<LogEntry> = Vec::new();
    for line in contents.lines() {
        match parse_line(line) {
            Ok(l) => log_entries.push(l),
            Err(e) => eprintln!("Failed to parse line: {}. Error: {}", line, e),
        }
    }

    Ok(log_entries)
}
