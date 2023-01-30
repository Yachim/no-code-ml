mod network;
mod utils;

use csv::Error;
use network::Network;
use std::fs;
use utils::functions::{activation::*, cost::mse_deriv};

fn main() -> Result<(), Error> {
    let digits_training_file =
        fs::read_to_string("./example_data/digits/train.csv").expect("error when loading the file");

    let mut rdr = csv::Reader::from_reader(digits_training_file.as_bytes());

    let mut inputs: Vec<Vec<f32>> = vec![];
    let mut expected: Vec<Vec<f32>> = vec![];

    for result in rdr.records() {
        let record = result?;

        let mut expected_for_sample = vec![0.0; 10];
        expected_for_sample[(&record[0]).parse::<usize>().unwrap()] = 1.0;
        expected.push(expected_for_sample);

        let mut inputs_for_sample = vec![0.0; (&record).len()];
        for pixel_i in 1..(&record).len() {
            inputs_for_sample[pixel_i] = (&record[pixel_i]).parse::<f32>().unwrap();
        }
        inputs.push(inputs_for_sample);
    }

    let mut net = Network::new(zip(inputs, expected), vec![16, 16], vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"], vec![&relu, &relu, &sigmoid], vec![&sigmoid_deriv, &sigmoid_deriv, &relu_deriv], &mse_deriv, 0.1, 10, )

    Ok(())
}
