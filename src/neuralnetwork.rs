use std::io::{Result, Error, ErrorKind};
use rand::Rng;


pub struct WeightLayer {
    input: usize,
    output: usize,
    bias: bool,
    learning_rate: f32,

    activation: fn(f32, usize) -> f32,
    back_activation: fn(f32, usize) -> f32,


    pub weights: Vec<f32>,
}

impl WeightLayer {
    pub fn new(input: usize, output: usize, bias: bool, learning_rate: f32, activation: fn(f32, usize) -> f32, back_activation: fn(f32, usize) -> f32) -> Result<WeightLayer> {
        let mut weights: Vec<f32> = Vec::new();

        for _ in 0..(input + bias as usize) {
            for _ in 0..output {
                weights.push(rand::thread_rng().gen::<f32>());
            }
        }

        Ok(
            WeightLayer { input, output, bias, learning_rate, weights, activation, back_activation }
        )
    }

    pub fn test(&self, input: &Vec<f32>) -> Result<Vec<f32>> {
        if input.len() != self.input {
            return Err(Error::new(ErrorKind::InvalidInput, "Invalid input size"));
        }

        let mut output: Vec<f32> = Vec::new();

        for x in 0..self.output {
            let mut output_neuron: f32 = 0.0;

            for y in 0..self.input {
                output_neuron += input[x] * self.weights[x * self.input + y];
            }

            if self.bias {
                output_neuron += 1.0 * self.weights[self.input * self.output + x];
            }

            output.push((self.activation)(output_neuron, self.input));
        }

        Ok(
            output
        )
    }

    pub fn learn(&mut self, input: &Vec<f32>, output: &Vec<f32>) -> Result<()> {
        if input.len() != self.input {
            return Err(Error::new(ErrorKind::InvalidInput, "Invalid input size"));
        }
        if output.len() != self.output {
            return Err(Error::new(ErrorKind::InvalidInput, "Invalid output size"));
        }

        let mut result: Vec<f32> = self.test(&input)?;

        for x in 0..self.output {
            for y in 0..self.input {
                self.weights[x * self.input + y] += self.learning_rate
                    * (result[x] - output[x])
                    * (self.back_activation)(result[x], self.input)
                    * input[y];
            }

            if self.bias {
                self.weights[self.input * self.output + x] += self.learning_rate
                    * (result[x] - output[x])
                    * (self.back_activation)(result[x], self.input)
                    * 1.0;
            }
        }

        Ok(())
    }
}
