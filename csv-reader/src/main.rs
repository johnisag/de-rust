use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use std::process;

// Simple CSV reading without Serde.
fn simple_read() -> Result<(), Box<dyn Error>> {
    println!("--- Simple CSV Read ---");
    
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

// Define a struct to represent each record.
#[derive(Debug, serde::Deserialize)]
struct Fruit {
    #[serde(rename = "Fruit")]    // Maps CSV header "Fruit" to field "name"
    name: String,
    #[serde(rename = "Quantity")] // Maps CSV header "Quantity" to field "quantity"
    quantity: u32,
}

// CSV reading with Serde deserialization.
fn serde_read() -> Result<(), Box<dyn Error>> {
    println!("--- Serde CSV Read ---");

    // Open the CSV file.
    let file = File::open("fruits.csv")?;

    // Build the CSV reader.
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    for result in rdr.deserialize() {
        // Get the record or return the error.
        let fruit: Fruit = result?;
        println!("{:?}", fruit);

        println!("Fruit: {}, Quantity: {}", fruit.name, fruit.quantity);
    }

    Ok(())
}

fn main() {
    if let Err(err) = simple_read() {
        println!("error running simple_read: {}", err);
        process::exit(1);
    }

    if let Err(err) = serde_read() {
        println!("error running serde_read: {}", err);
        process::exit(1);
    }
}