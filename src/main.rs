mod data;

use data::deserialize;

use std::io::Result;
use serde::Deserialize;

#[derive(Deserialize)]
struct Data {
    input: Vec<u8>,
    output: u8
}

fn load_dataset<T: for<'de> Deserialize<'de>>(path: &str, name: &str) -> Result<Vec<T>> {
    println!("Extracting data from a dataset from {path}/{name}...");
    let dataset: Vec<T> = deserialize::<Vec<T>>(path, name)?;
    println!("Done! Amount of educational material {}", dataset.len());
    println!("");
    
    Ok(dataset)
}

fn main() -> Result<()> {
    let path: &str = "data/mnist";
    let learn_dataset_name: &str = "learn-0";
    let test_dataset_name: &str = "test-0";

    let learn: Vec<Data> = load_dataset::<Data>(path, learn_dataset_name)?;
    let test: Vec<Data> = load_dataset::<Data>(path, test_dataset_name)?;
    println!("");

    // code
    println!("Input: {:?}", learn[0].input);
    println!("Output: {}", learn[0].output);

    Ok(())
}
