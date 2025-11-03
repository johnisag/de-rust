use csv::Writer;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let data = [
        ("apple", 3),
        ("banana", 5),
        ("orange", 2),
    ];

    // Create a CSV writer and write data to a file
    let mut wtr = Writer::from_path("fruits.csv")?;
    
    // Write header
    wtr.write_record(&["Fruit", "Quantity"])?;

    // Write records
    for (fruit, quantity) in &data {
        // print data to console
        println!("Writing record: {}, {}", fruit, quantity);

        // Write each record to the CSV file
        wtr.write_record(&[*fruit, &quantity.to_string()])?;
    }

    // Flush the writer to ensure all data is written to the file
    wtr.flush()?;

    Ok(())
}
