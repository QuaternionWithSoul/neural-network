use std::io::Result;
use rand::{Rng, rngs::ThreadRng};


#[derive(Clone)]
pub struct DenseLayer {
    pub weights: Vec<Vec<f32>>,
    pub bias_weights: Vec<f32>,
    pub input_size: usize,
    pub output_size: usize,
    activation: fn(f32) -> f32
}

impl DenseLayer {
    pub fn new(input_size: usize, output_size: usize, activation: fn(f32) -> f32) -> Result<DenseLayer> {
        let mut rng: ThreadRng = rand::thread_rng();

        let mut weights: Vec<Vec<f32>> = Vec::new();
        for _ in 0..input_size {
            let mut input_weights: Vec<f32> = Vec::new();
            for _ in 0..output_size {
                input_weights.push(rng.gen_range(-1.0..1.0));
            }
            weights.push(input_weights);
        }

        let mut bias_weights: Vec<f32> = Vec::new();
        for _ in 0..output_size {
            bias_weights.push(rng.gen_range(-1.0..1.0));
        }

        Ok(
            DenseLayer { weights, bias_weights, input_size, output_size, activation }
        )
    }

    pub fn prediction(&self, inputs: Vec<f32>) -> Result<Vec<f32>> {
        let mut outputs: Vec<f32> = Vec::new();

        for y in 0..self.output_size {
            let mut neuron_before_activation: f32 = 0.0;
            for x in 0..self.input_size {
                neuron_before_activation += inputs[x] * self.weights[x][y];
            }
            neuron_before_activation += self.bias_weights[y];

            outputs.push((self.activation)(neuron_before_activation));
        }

        Ok( outputs )
    }

    pub fn error_backpropagation(&mut self, inputs: Vec<f32>, outputs: Vec<f32>) -> Result<DenseLayer> {
        Ok( self.clone() )
    }
}
