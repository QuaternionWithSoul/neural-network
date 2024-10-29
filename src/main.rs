mod data;

use data::deserialize;

use std::io::Result;
use serde::Deserialize;

#[derive(Deserialize)]
struct Data {
    input: Vec<u8>,
    output: u8
}

fn main() -> Result<()> {
    let path: &str = "data/mydataset";
    let name: &str = "learn-0";

    println!("Extracting data from a dataset...");
    let dataset: Vec<Data> = deserialize::<Vec<Data>>(path, name)?;
    println!("Done! Amount of educational material {}", dataset.len());
    println!("");

    for index in 0..dataset.len() {
        println!("Input: {:?}", dataset[index].input);
        println!("Output: {}", dataset[index].output);
        println!("");
    }

    Ok(())
}
