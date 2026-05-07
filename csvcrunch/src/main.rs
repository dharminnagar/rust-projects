use std::{sync::{Arc, mpsc}, thread};
use anyhow::{Context, Result};
use std::env;

mod utils;
use utils::csv::parse_csv;

#[derive(Debug, Clone)]
struct ChunkStats {
    count: usize,
    sum: f64,
    min: f64,
    max: f64,
    avg: f64,
}

// get a chunk of rows, compute stats for each, return the results
fn compute_chunk(chunk: &[Vec<String>]) -> ChunkStats {
    let mut stats = ChunkStats {
        count: 0,
        sum: 0.0,
        min: f64::INFINITY,
        max: f64::NEG_INFINITY,
        avg: 0.0,
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

// use mpsc for communication we could just return the stats from each thread and aggregate them in the main thread
fn spawn_and_aggregate(parsed_csv: Arc<Vec<Vec<String>>>, chunk_size: usize, num_threads: usize) -> Vec<ChunkStats> {

    let (tx, rx) = mpsc::channel();

    for i in 0..num_threads {
        let tx = tx.clone();
        let start = i * chunk_size;
        let end = (start + chunk_size).min(parsed_csv.len());
        let chunk = Arc::clone(&parsed_csv);

        thread::spawn(move || {
            let result = compute_chunk(&chunk[start..end]);
            tx.send(result).unwrap();
        });
    }

    drop(tx); // close the channel so that the iterator will end when all threads have finished

    let stats: Vec<ChunkStats> = rx.iter().collect();

    return stats;
}


fn merge_stats(stats: Vec<ChunkStats>) -> Result<ChunkStats> {
    stats.into_iter().reduce(
        |acc, chunk| ChunkStats {
            count: acc.count + chunk.count,
            sum: acc.sum + chunk.sum,
            min: acc.min.min(chunk.min),
            max: acc.max.max(chunk.max),
            avg: 0.0,
        }
    ).context("The stats vector was empty")

    // alternatively, we could do it with a simple loop:
    // for stats in &stats {
    //     final_stats.count += stats.count;
    //     final_stats.sum += stats.sum;
    //     final_stats.min = final_stats.min.min(stats.min);
    //     final_stats.max = final_stats.max.max(stats.max);
    // }
}

fn main() -> Result<()> {
    println!("gm, this is a csv parser\n");

    let input_args: Vec<String> = env::args().collect();

    let file_path = input_args.get(1).context("usage: csvcrunch <file_path>")?;

    let parsed_csv = match parse_csv(file_path) {
        Ok(data) => data,
        Err(e) => {
            return Err(anyhow::anyhow!("Failed to parse CSV file {}", e));
        }
    };
    let parsed_csv = Arc::new(parsed_csv);
    let num_threads = std::thread::available_parallelism()?.get(); // get the number of available CPU cores

    let chunk_size = (parsed_csv.len() + num_threads - 1) / num_threads; // calculate the chunk size using "ceiling" division

    let stats = spawn_and_aggregate(parsed_csv, chunk_size, num_threads);

    let final_stats = match merge_stats(stats) {
        Ok(stats) => {
            let mut stats = stats;
            stats.avg = stats.sum / stats.count as f64;
            stats
        },
        Err(e) => {
            println!("Error merging stats: {}", e);
            return Err(anyhow::anyhow!("Failed to merge stats"));
        }
    };

    println!("final stats: count {} | min: {} | max {} | sum {} | avg {}", final_stats.count, final_stats.min, final_stats.max, final_stats.sum, final_stats.avg);

    Ok(())
}
