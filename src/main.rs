mod network;
mod utils;

use csv::Error;
use network::Network;
use std::iter::zip;
use utils::functions::{activation::*, cost::mse_deriv};

fn main() -> Result<(), Error> {
    // https://www.kaggle.com/competitions/digit-recognizer/data
    let mut train_rdr = csv::Reader::from_path("src/example_data/digits/train.csv")?;

    let mut inputs: Vec<Vec<f32>> = vec![];
    let mut expected: Vec<Vec<f32>> = vec![];

    for result in train_rdr.records() {
        let record = result?;

        let mut expected_for_sample = vec![0.0; 10];
        expected_for_sample[record[0].parse::<usize>().unwrap()] = 1.0;
        expected.push(expected_for_sample);

        let mut inputs_for_sample = vec![0.0; record.len() - 1];
        for pixel_i in 1..record.len() - 1 {
            inputs_for_sample[pixel_i] = record[pixel_i].parse::<f32>().unwrap();
        }
        inputs.push(inputs_for_sample);
    }

    let mut net = Network::new(
        zip(inputs, expected).collect(),
        vec![16, 16],
        vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
        vec![&relu, &relu, &sigmoid],
        vec![&relu_deriv, &relu_deriv, &sigmoid_deriv],
        &mse_deriv,
        0.3,
        2,
        100,
    );

    net.train();

    let mut test_rdr = csv::Reader::from_path("src/example_data/digits/test.csv")?;
    let mut test_wtr = csv::Writer::from_path("src/example_data/digits/out.csv")?;

    test_wtr.write_record(&["ImageId", "Label"])?;

    for (i, result) in test_rdr.records().enumerate() {
        let record = result?;

        let mut test_input = vec![0.0; record.len()];
        for pixel_i in 1..record.len() - 1 {
            test_input[pixel_i] = record[pixel_i].parse::<f32>().unwrap();
        }

        net.predict(test_input);
        let val = net.get_best_output().0;

        test_wtr.write_record(&[(i + 1).to_string(), val.to_string()])?;
    }
    test_wtr.flush()?;

    Ok(())
}
