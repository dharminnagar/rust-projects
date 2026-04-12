use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::{SystemTime, UNIX_EPOCH};

const LEVELS: [&str; 4] = ["INFO", "DEBUG", "WARN", "ERROR"];

const INFO_MESSAGES: [&str; 8] = [
    "Server started on port 8080",
    "Scheduled job completed successfully",
    "Connected to database cluster",
    "User session created",
    "Cache warmed for homepage",
    "Background sync completed",
    "Health check passed",
    "Configuration loaded from disk",
];

const DEBUG_MESSAGES: [&str; 8] = [
    "Parsed request payload",
    "Cache miss for key user_profile",
    "Retry attempt for upstream request",
    "Token validation took 4 ms",
    "Loaded 32 feature flags",
    "Worker heartbeat recorded",
    "Opened pooled database connection",
    "Rate limiter counter incremented",
];

const WARN_MESSAGES: [&str; 8] = [
    "High memory usage detected",
    "Slow query exceeded 500 ms",
    "Circuit breaker half-open state",
    "Disk usage reached 85 percent",
    "API response latency trending up",
    "Login retries approaching threshold",
    "Temporary network instability detected",
    "Fallback cache path engaged",
];

const ERROR_MESSAGES: [&str; 8] = [
    "Failed to connect to database",
    "Unhandled exception in request handler",
    "Payment provider timeout",
    "Failed to persist audit event",
    "Queue publish operation failed",
    "TLS handshake failed",
    "Unable to refresh auth token",
    "File write operation denied",
];

struct Lcg {
    state: u64,
}

impl Lcg {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u32(&mut self) -> u32 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1);
        (self.state >> 32) as u32
    }

    fn range_inclusive(&mut self, min: u32, max: u32) -> u32 {
        let span = max - min + 1;
        min + (self.next_u32() % span)
    }
}

fn timestamp_for_offset(seconds: u32) -> String {
    // Uses a fixed date and increments seconds to keep output deterministic in format.
    let base_hour = 10;
    let base_minute = 32;
    let base_second = 1;

    let total = base_hour * 3600 + base_minute * 60 + base_second + seconds;
    let hour = (total / 3600) % 24;
    let minute = (total % 3600) / 60;
    let second = total % 60;

    format!("2024-01-15 {:02}:{:02}:{:02}", hour, minute, second)
}

fn choose_level(rng: &mut Lcg) -> &'static str {
    // Weighted distribution: INFO 50%, DEBUG 25%, WARN 15%, ERROR 10%
    let roll = rng.range_inclusive(1, 100);
    match roll {
        1..=50 => LEVELS[0],
        51..=75 => LEVELS[1],
        76..=90 => LEVELS[2],
        _ => LEVELS[3],
    }
}

fn choose_message<'a>(rng: &mut Lcg, level: &str) -> &'a str {
    let idx = rng.range_inclusive(0, 7) as usize;
    match level {
        "INFO" => INFO_MESSAGES[idx],
        "DEBUG" => DEBUG_MESSAGES[idx],
        "WARN" => WARN_MESSAGES[idx],
        _ => ERROR_MESSAGES[idx],
    }
}

fn main() -> std::io::Result<()> {
    let output = env::args()
        .nth(1)
        .unwrap_or_else(|| "sample.log".to_string());

    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0x5EED);

    let mut rng = Lcg::new(seed);
    let line_count = rng.range_inclusive(50, 100);

    let file = File::create(&output)?;
    let mut writer = BufWriter::new(file);

    for i in 0..line_count {
        let level = choose_level(&mut rng);
        let message = choose_message(&mut rng, level);
        let ts = timestamp_for_offset(i);

        // Keep alignment similar to common server logs.
        writeln!(writer, "{} {:<5} {}", ts, level, message)?;
    }

    writer.flush()?;
    println!("Generated {} log lines in {}", line_count, output);
    Ok(())
}
