use std::error::Error;
use std::fs::File;
use std::sync::Arc;

use arrow::array::{Int32Array, StringArray, Array, RecordBatchReader};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;

use parquet::arrow::{ArrowWriter, arrow_reader::ParquetRecordBatchReaderBuilder};
use parquet::file::properties::WriterProperties;

fn main() {
    // Write a Parquet file
    if let Err(e) = write_parquet("example.parquet") {
        eprintln!("Error writing Parquet file: {}", e);
        return; 
    }

    // Read the Parquet file
    if let Err(e) = read_parquet("example.parquet") {
        eprintln!("Error reading Parquet file: {}", e);
        return;
    }   
}

// Function to write a Parquet file
fn write_parquet(path: &str) -> Result<(), Box<dyn Error>> {
    // Define schema
    let schema = Arc::new(Schema::new(vec![
        Field::new("name", DataType::Utf8, false),
        Field::new("age", DataType::Int32, true), // nullable field
    ]));

    // Create sample data
    let names = StringArray::from(vec!["Alice", "Bob", "Charlie"]);
    let ages = Int32Array::from(vec![Some(30), None, Some(25)]);

    // Create a record batch
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(names),
            Arc::new(ages),
        ],
    )?;

    // Write the record batch to a Parquet file
    let file = File::create(path)?;
    let props = WriterProperties::builder().build();
    let mut writer = ArrowWriter::try_new(file, schema, Some(props))?;
    writer.write(&batch)?;
    writer.finish()?;

    println!("Parquet file written to {}", path);

    Ok(())
}

// Function to read a Parquet file
fn read_parquet(path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(path)?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
    let reader = builder.build()?;

    println!("Reading Parquet file: {}", path);
    println!("Schema: {:?}", reader.schema());

    // Read the record batches from the Parquet file
    for batch_result in reader {
        let batch = batch_result?;
        println!("\nRead batch with {} rows", batch.num_rows());

        // Print schema info
        for (i, field) in batch.schema().fields().iter().enumerate() {
            println!("Column {}: {} ({})", i, field.name(), field.data_type());
        }

        // Print the data
        for row_idx in 0..batch.num_rows() {
            print!("Row {}: ", row_idx);
            
            // Name column (column 0)
            let name_array = batch.column(0).as_any().downcast_ref::<StringArray>().unwrap();
            print!("name='{}', ", name_array.value(row_idx));
            
            // Age column (column 1) - handle nullable values
            let age_array = batch.column(1).as_any().downcast_ref::<Int32Array>().unwrap();
            if age_array.is_null(row_idx) {
                print!("age=null");
            } else {
                print!("age={}", age_array.value(row_idx));
            }
            
            println!();
        }
    }

    Ok(())
}
