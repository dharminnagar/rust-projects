pub mod log;

use log::{LogEntry, LogLevel};
use std::collections::HashMap;

// Summary example
// ERROR   12 entries
// WARN     8 entries
// INFO    43 entries
pub fn summarize(entries: Vec<LogEntry>) -> HashMap<LogLevel, Vec<LogEntry>> {
    let mut summary: HashMap<LogLevel, Vec<LogEntry>> = HashMap::new();

    for entry in entries {
        summary.entry(entry.level.clone()).or_default().push(entry);
    }
    
    summary
}

pub fn print_summary(map: &HashMap<LogLevel, Vec<LogEntry>>, filter: Option<&String>)  {
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
