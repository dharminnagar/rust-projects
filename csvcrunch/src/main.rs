use std::{sync::Arc, thread};

mod utils;
use utils::csv::parse_csv;

struct ChunkStats {
    count: usize,
    sum: f64,
    min: f64,
    max: f64,
}

// get a chunk of rows, compute stats for each, return the results
fn compute(chunk: &[Vec<String>]) -> ChunkStats {
    let mut stats = ChunkStats {
        count: 0,
        sum: 0.0,
        min: f64::INFINITY,
        max: f64::NEG_INFINITY,
    };

    for row in chunk {
        if let Ok(value) = row[2].parse::<f64>() {            
            stats.count += 1;
            stats.sum += value;
            stats.min = stats.min.min(value);
            stats.max = stats.max.max(value);
        }
    }

    return stats;
}

fn main() {
    println!("gm, this is a csv parser\n");

    let rows = Arc::new(parse_csv("/Users/dharmin/Dev/Languages/Rust/rust-projects/csvcrunch/data/test_data.csv").unwrap());
    let num_threads = std::thread::available_parallelism().unwrap().get(); // get the number of available CPU cores

    let chunk_size = (rows.len() + num_threads - 1) / num_threads; // calculate the chunk size using "ceiling" division
    
    println!("Number of threads: {}", num_threads);
    println!("Number of rows: {}", rows.len());

    let mut handles = Vec::new();

    for i in 0..num_threads {
        let start = i * chunk_size;
        let end = (start + chunk_size).min(rows.len());
        let chunk = Arc::clone(&rows);

        let thread = thread::spawn(move || compute(&chunk[start..end]));
        handles.push(thread);
    }

    for (i, handle) in handles.into_iter().enumerate() {
        let stats = handle.join().unwrap();
        println!("Thread {}: count {} | min: {} | max {} | sum {}", i, stats.count, stats.min, stats.max, stats.sum);
    }
}
