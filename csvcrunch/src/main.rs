mod utils;
use utils::csv::parse_csv;

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
}
