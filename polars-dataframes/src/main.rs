use polars::prelude::*;

fn create_sample_df() -> DataFrame {
    // Create a sample DataFrame with some data
    let df = df! (
        "Name" => &["Alice", "Bob", "Charlie"],
        "Age" => &[25, 30, 35],
        "City" => &["New York", "Los Angeles", "Chicago"]
    ).unwrap();

    // Return the created DataFrame
    df
}

fn main() {
    // Create a sample DataFrame and print it
    let df = create_sample_df();
    println!("{:?}", df);
}
