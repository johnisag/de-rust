use polars::prelude::*;

fn create_sample_df() -> DataFrame {
    // Create a sample DataFrame with some data
    let df = df! (
        "Name" => ["Alice", "Bob", "Charlie"],
        "Age" => [25, 30, 35],
        "City" => ["New York", "Los Angeles", "Chicago"]
    ).unwrap();

    // Return the created DataFrame
    df
}

fn read_csv_to_df(file_path: &str) -> PolarsResult<DataFrame> {
    // Read a CSV file into a DataFrame using CsvReadOptions
    let df = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some(file_path.into()))?
        .finish()?;

    Ok(df)
}

fn read_csv_to_lazy_df(file_path: &str) -> PolarsResult<DataFrame> {
    // Read a CSV file into a LazyFrame and then collect it into a DataFrame
    let q  = LazyCsvReader::new(PlPath::new(file_path))
    .with_has_header(true)
    .finish()?;

    let df = q.collect()?;

    Ok(df)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Specify the path to the CSV file
    let file_path = "data/data.csv";

    
    // Create a sample DataFrame and print it
    println!("Polars Dataframe Sample");
    
    let df = create_sample_df();
    println!("{:?}", df);

    // Read a CSV file into a DataFrame
    println!("\nReading CSV into DataFrame:");

    let csv_df = read_csv_to_df(file_path);
    match csv_df {
        Ok(df) => println!("CSV DataFrame: {:?}", df.head(Some(5))),
        Err(e) => eprintln!("Error reading CSV: {}", e),
    }

    // Read a CSV file into a LazyFrame
    println!("\nReading CSV into LazyFrame and collecting into DataFrame:");
    let df_result = read_csv_to_lazy_df(file_path);
    match &df_result {
        Ok(df) => println!("CSV DataFrame: {:?}", df.head(Some(5))),
        Err(e) => eprintln!("Error reading CSV: {}", e),
    }

    // Perform some basic operations using LazyFrame
    println!("\nPerforming operations using LazyFrame:");

    let agg_df = df_result.unwrap()
            .lazy()
            .select([col("Country Name"), col("2018")])
            .filter(col("2018").gt(lit(28)))
            .group_by([col("Country Name")])
            .agg([col("2018").count().alias("Number_of_Entries")])
            .sort(["Number_of_Entries"], SortMultipleOptions::default().with_order_descending(true))
            .collect();

    match agg_df {
        Ok(result) => println!("Aggregated DataFrame: {:?}", result.head(Some(5))),
        Err(e) => eprintln!("Error in aggregation: {}", e),
    }

    // Single pipeline to read CSV and perform operations
    println!("\nSingle pipeline to read CSV and perform operations:");
    let single_pipeline_df = LazyCsvReader::new(PlPath::new(file_path))
        .with_has_header(true)
        .finish()?
        .select([col("Country Name"), col("2018")])
            .filter(col("2018").gt(lit(28)))
            .group_by([col("Country Name")])
            .agg([col("2018").count().alias("Number_of_Entries")])
            .sort(["Number_of_Entries"], SortMultipleOptions::default().with_order_descending(true))
            .collect();

    match single_pipeline_df {
        Ok(result) => println!("Single pipeline DataFrame: {:?}", result.head(Some(5))),
        Err(e) => eprintln!("Error in single pipeline: {}", e),
    }

    Ok(())
}
