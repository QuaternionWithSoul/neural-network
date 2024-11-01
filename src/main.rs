mod data;
mod neuralnetwork;

use data::deserialize;
use neuralnetwork::WeightLayer;

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

fn f(x: f32, i: usize) -> f32 {
    (x.abs() / i as f32).sin().abs() / 0.84147096
}

fn back_f(x: f32, i: usize) -> f32 {
    (x * 0.84147096).asin().abs() * i as f32
}

fn main() -> Result<()> {
    let path: &str = "data/mydataset";
    let learn_dataset_name: &str = "learn-0";
    let test_dataset_name: &str = "test-0";

    let learn: Vec<Data> = load_dataset::<Data>(path, learn_dataset_name)?;
    let test: Vec<Data> = load_dataset::<Data>(path, test_dataset_name)?;
    println!("");

    // code
    let mut wl: WeightLayer = WeightLayer::new(3, 1, true, 0.1, f, back_f)?;

    println!("{:?}", wl.weights);
    println!("");

    for _ in 0..10 {
        for data in &learn {
            let input: Vec<f32> = data.input.iter().map(|&x| x as f32).collect();
            let output: Vec<f32> = vec![data.output as f32];
    
            wl.learn(&input, &output)?;
        }
    }

    for data in &learn {
        let input: Vec<f32> = data.input.iter().map(|&x| x as f32).collect();
        let output: Vec<f32> = vec![data.output as f32];
    
        println!("Input {:?}", &input);
        println!("Result {:?}", wl.test(&input)?);
        println!("Output {:?}", &output);
        println!("");
    }

    for data in &test {
        let input: Vec<f32> = data.input.iter().map(|&x| x as f32).collect();
        let output: Vec<f32> = vec![data.output as f32];
    
        println!("Input {:?}", &input);
        println!("Result {:?}", wl.test(&input)?);
        println!("Output {:?}", &output);
        println!("");
    }

    println!("{:?}", wl.weights);
    println!("");

    Ok(())
}
