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
    let path: &str = "data/mydataset";

    let learn: Vec<Data> = load_dataset::<Data>(path, "learn-0")?;
    let test: Vec<Data> = load_dataset::<Data>(path, "test-0")?;
    println!("");

    println!("Contents of the learn dataset:\n");
    for index in 0..learn.len() {
        println!("Input: {:?}", learn[index].input);
        println!("Output: {}", learn[index].output);
        println!("");
    } println!("");

    println!("Contents of the test dataset:\n");
    for index in 0..test.len() {
        println!("Input: {:?}", test[index].input);
        println!("Output: {}", test[index].output);
        println!("");
    } println!("");

    Ok(())
}
