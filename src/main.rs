mod functions;
mod network;
mod utils;

use csv::Error;
use functions::{
    activation::*,
    cost::MSE,
    input_normalizations::{NORMALIZATION, NO_NORMALIZATION},
};
use network::Network;
use utils::csv_data::{CsvDataLoader, Label};

fn digits() -> Result<(), Error> {
    // https://www.kaggle.com/competitions/digit-recognizer/data
    let csv_loader = CsvDataLoader::new();

    let labels = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let training_set = csv_loader.load_labeled_data(
        "src/example_data/digits/train.csv",
        Label::SingleLabelClassification(0, &labels),
    )?;

    let mut net = Network::new(
        784,
        vec![16, 16],
        labels,
        vec![&RELU, &RELU, &SIGMOID],
        &MSE,
        &NORMALIZATION,
    );
    net.log_costs = true;
    net.log_epochs = true;

    net.train(training_set, 100, 0.001, 10);

    let test_data = csv_loader.load_unlabeled_data("src/example_data/digits/test.csv")?;

    let mut test_wtr = csv::Writer::from_path("src/example_data/digits/out_relu.csv")?;
    test_wtr.write_record(&["ImageId", "Label"])?;

    for (i, test_input) in test_data.iter().enumerate() {
        net.predict(test_input.to_vec());
        let val = net.get_best_output().0;

        test_wtr.write_record(&[(i + 1).to_string(), val.to_string()])?;
    }

    test_wtr.flush()?;

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
        vec![&SIGMOID],
        &MSE,
        &NO_NORMALIZATION,
    );
    net.log_costs = true;

    net.train(training_set, 10000, 1.0, 4);

    let test_set = vec![1.0, 0.0, 0.0];
    net.predict(test_set);

    let out_map = net.get_output();
    let out = out_map.get("res").unwrap();

    println!("output: {out}");
}

fn main() {
    medium_numbers();
    println!("\n\n");
    digits().unwrap();
}
