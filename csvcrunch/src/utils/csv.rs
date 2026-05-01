use std::error::Error;

pub fn parse_csv(file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(file_path)?;
    let mut csv_data: Vec<Vec<String>> = Vec::new();

    for result in reader.records() {
        let result = result?;

        let record = result.iter().map(|field| field.to_string()).collect::<Vec<String>>();
        csv_data.push(record);
    }

    println!("CSV data parsed successfully: {:?}", csv_data);
    
    Ok(())
}