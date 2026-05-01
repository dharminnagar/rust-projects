mod utils;
use utils::csv::parse_csv;

fn main() {
    println!("gm, this is a csv parser");

    if let Err(e) = parse_csv("/Users/dharmin/Dev/Languages/Rust/rust-projects/csvcrunch/data/test_data.csv") {
        eprintln!("Error reading CSV file: {}", e);
    }
}
