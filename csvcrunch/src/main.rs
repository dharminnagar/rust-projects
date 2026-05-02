use std::thread;

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

    let rows = parse_csv("/Users/dharmin/Dev/Languages/Rust/rust-projects/csvcrunch/data/test_data.csv").unwrap();
    let num_threads = std::thread::available_parallelism().unwrap().get(); // get the number of available CPU cores

    let chunk_size = (rows.len() + num_threads - 1) / num_threads; // calculate the chunk size using "ceiling" division
    let chunks: Vec<_> = rows.chunks(chunk_size).collect();
    // `chunks_exact` can be used if you want to ignore any remaining rows that don't fit into a full chunk, but `chunks` will include them in the last chunk.
    
    println!("Number of threads: {}", num_threads);
    println!("Number of rows: {}", rows.len());
    println!("Number of chunks: {}", chunks.len());
    
    for (i, chunk) in chunks.iter().enumerate() {
        println!("Chunk {}: {} rows", i + 1, chunk.len());
    }

    let total_rows: usize = chunks.iter().map(|c| c.len()).sum();
    println!("Total rows across chunks: {}", total_rows);

    let mut handles = Vec::new();

    for i in 0..chunks.len() {
        let chunk = chunks[i].to_vec();
        let thread = thread::spawn(move || compute(&chunk));
        handles.push(thread);
    }

    for (i, handle) in handles.into_iter().enumerate() {
        let stats = handle.join().unwrap();
        println!("Thread {}: count {} | min: {} | max {} | sum {}", i, stats.count, stats.min, stats.max, stats.sum);
    }
}
