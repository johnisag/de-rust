use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use std::process;

fn simple_read() -> Result<(), Box<dyn Error>> {
    // Open the CSV file.
    let file = File::open("fruits.csv")?;

    // Build the CSV reader.
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    for result in rdr.records() {
        // Get the record or return the error.
        let record = result?;
        println!("{:?}", record);

        // Unpack to specific fields if needed.
        let name = &record[0];
        let quantity: u32 = record[1].parse()?;
        println!("Fruit: {}, Quantity: {}", name, quantity);
    }

    Ok(())
}

fn main() {
    if let Err(err) = simple_read() {
        println!("error running simple_read: {}", err);
        process::exit(1);
    }
}