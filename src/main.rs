mod data;

use data::deserialize;

use std::io::Result;
use serde::Deserialize;

#[derive(Deserialize)]
struct Data {
    inputs: Vec<u8>,
    outputs: u8
}

fn main() -> Result<()> {
    let path: &str = "data/mnist";
    let name: &str = "learn-0";

    println!("Extracting data from a dataset...");
    let data: Vec<Data> = deserialize::<Vec<Data>>(path, name)?;
    println!("Done!");
    println!("");

    for index in 0..1 {
        println!("Inputs: {:?}", data[index].inputs);
        println!("Outputs: {}", data[index].outputs);
    }

    Ok(())
}
