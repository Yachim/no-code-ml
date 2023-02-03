mod network;
mod utils;

use csv::Error;
use network::Network;
use std::iter::zip;
use utils::functions::{
    activation::*,
    cost::{mse, mse_deriv},
};

// wrong results - only 1s in output; outputs only 9
fn digits() -> Result<(), Error> {
    // https://www.kaggle.com/competitions/digit-recognizer/data
    let mut train_rdr = csv::Reader::from_path("src/example_data/digits/train.csv")?;

    let mut inputs: Vec<Vec<f32>> = vec![];
    let mut expected: Vec<Vec<f32>> = vec![];

    for result in train_rdr.records() {
        let record = result?;

        let mut expected_for_sample = vec![0.0; 10];
        expected_for_sample[record[0].parse::<usize>().unwrap()] = 1.0;
        expected.push(expected_for_sample);

        let mut inputs_for_sample = vec![];
        for pixel_i in 1..record.len() {
            inputs_for_sample.push(record[pixel_i].parse::<f32>().unwrap());
        }
        inputs.push(inputs_for_sample);
    }

    let training_set = zip(inputs, expected).collect();

    let mut net = Network::new(
        784,
        vec![16, 16],
        vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
        vec![&relu, &relu, &sigmoid],
        vec![&relu_deriv, &relu_deriv, &sigmoid_deriv],
        &mse_deriv,
        &mse,
    );
    net.log_costs = true;
    net.log_epochs = true;

    net.train(training_set, 100, 0.001, 10);

    let mut test_rdr = csv::Reader::from_path("src/example_data/digits/test.csv")?;
    let mut test_wtr = csv::Writer::from_path("src/example_data/digits/out_relu.csv")?;

    test_wtr.write_record(&["ImageId", "Label"])?;

    for (i, result) in test_rdr.records().enumerate() {
        let record = result?;

        let mut test_input = vec![];
        for pixel in &record {
            test_input.push(pixel.parse::<f32>().unwrap());
        }

        net.predict(test_input);
        let val = net.get_best_output().0;

        test_wtr.write_record(&[(i + 1).to_string(), val.to_string()])?;
    }

    test_wtr.flush()?;

    net.debug();

    Ok(())
}

fn medium_numbers() {
    // https://medium.com/technology-invention-and-more/how-to-build-a-simple-neural-network-in-9-lines-of-python-code-cc8f23647ca1
    let training_set = vec![
        (vec![0.0, 0.0, 1.0], vec![0.0]),
        (vec![1.0, 1.0, 1.0], vec![1.0]),
        (vec![1.0, 0.0, 1.0], vec![1.0]),
        (vec![0.0, 1.0, 1.0], vec![0.0]),
    ];
    let mut net = Network::new(
        3,
        vec![],
        vec!["res"],
        vec![&sigmoid],
        vec![&sigmoid_deriv],
        &mse_deriv,
        &mse,
    );

    net.train(training_set, 10000, 1.0, 1);

    let test_set = vec![1.0, 0.0, 0.0];
    net.predict(test_set);

    let out_map = net.get_output();
    let out = out_map.get("res").unwrap();

    println!("{out}");
    net.debug();
}

fn main() {
    digits().unwrap();
}
