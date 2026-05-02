# csvcrunch

Multithreaded CSV aggregator. Reads large CSV, splits rows across N worker threads (N = CPU cores), each thread aggregates its chunk, main thread merges results.

## What It Does

Given a CSV with rows and numeric columns:
- Parses CSV into `Vec<Vec<String>>`
- Splits rows into N chunks (N = available CPU cores)
- Spawns N threads, each aggregates its chunk independently
- Computes per-chunk: row count, min/max/sum of numeric column
- Main thread collects all results, merges, prints final stats

**Output:** total rows, min, max, average, top 5 most frequent values in column.

## Architecture

### Step 1: Parse CSV
Read CSV file line by line using `csv` crate. Returns `Vec<Vec<String>>` (all rows, all columns).

### Step 2: Split into Chunks
Compute chunk size: `(rows.len() + num_threads - 1) / num_threads` (ceiling division).
Use `.chunks(chunk_size)` to split. Last chunk may be smaller—that's OK.
Result: N chunks distributed across N threads.

### Step 3: Spawn Threads
For each chunk:
- Spawn thread with `thread::spawn(move || aggregate_chunk(chunk))`
- `move` transfers chunk ownership into thread—no shared references
- Thread computes aggregation: iterate rows, parse numeric column, track min/max/sum
- Thread returns `ChunkStats` struct
- Store `JoinHandle` to collect result later

### Step 4: Channel Results
Each thread sends its aggregated result through `mpsc` channel.
Main thread receives all results, collects them.
Threads join; all results collected.

### Step 5: Merge & Print
Main thread merges all `ChunkStats`:
- Sum all counts → total rows
- Find global min/max across all chunks
- Sum all sums, divide by total count → average
- Aggregate value frequencies, find top 5

## Concurrency Model

**No locks.** Each thread has immutable slice of rows—reads only, no mutation.

**Send/Sync:**
- `Vec<Vec<String>>` is `Send` because `String` is `Send`
- Chunk slice `&[Vec<String>]` is `Send`—references to sendable data
- Thread closure captures chunk via `move`—takes ownership, no shared state

**Channels:**
- `mpsc` = multiple producer, single consumer
- Main thread creates channel, clones sender for each worker
- Each worker sends its `ChunkStats` back through channel
- Main thread receives all results, processes

## Building & Running

```bash
cargo build --release
cargo run
```

Generates test CSV (50 rows) in `data/test_data.csv`, aggregates it.

Test with larger CSV:
```bash
# Generate 100k rows
node data/generate_csv.js
cargo run
```

## Design Decisions

- **Chunks over thread pools:** Simpler. Fixed N chunks = fixed N threads. No queue/scheduler overhead.
- **Immutable chunks:** No locks needed. Each thread reads its slice independently.
- **Result aggregation in main:** Keep aggregation logic centralized, easier to verify.
- **Uneven final chunk:** OK. Last chunk may have fewer rows. Threads finish when done.

## Next Steps

1. Add progress reporting via channel
2. Benchmark with `cargo bench`
3. Learn `cargo test`, `cargo doc`, `cargo fmt`, `cargo clippy`
4. Add error handling (thiserror)
