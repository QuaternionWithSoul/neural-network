mod data;
mod neuralnetwork;

use data::deserialize;
use neuralnetwork::DenseLayer;

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

fn relu(x: f32) -> f32 {
    x .rem_euclid(1.0)
}

fn main() -> Result<()> {
    let path: &str = "data/mydataset";
    let learn_dataset_name: &str = "learn-0";
    let test_dataset_name: &str = "test-0";

    let learn: Vec<Data> = load_dataset::<Data>(path, learn_dataset_name)?;
    let test: Vec<Data> = load_dataset::<Data>(path, test_dataset_name)?;
    println!("");

    // code
    let dl: DenseLayer = DenseLayer::new(3, 1, relu)?;

    println!("Input: {:?}", vec![1.0, 0.0, 0.0]);
    println!("Prediction: {:?}", dl.prediction(vec![1.0, 0.0, 0.0])?);
    println!("");
    println!("Neuron weights: {:?}", dl.weights);
    println!("Bias neuron weights: {:?}", dl.bias_weights);

    Ok(())
}
