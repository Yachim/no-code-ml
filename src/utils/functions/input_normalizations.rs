// https://www.baeldung.com/cs/normalizing-inputs-artificial-neural-network
use crate::network::Network;

pub struct NormalizationFn<'a> {
    pub function: &'a dyn Fn(&mut Network),
    pub description: &'a str,
    /// latex formula
    pub formula: &'a str,
}

fn no_func(network: &mut Network) {}
pub const NO_NORMALIZATION: NormalizationFn = NormalizationFn {
    function: &no_func,
    description: "",
    formula: "",
};

/// mutates the network so that the input is normalized
fn normalization(network: &mut Network) {
    let input = network.input.to_owned();

    let max = input
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    let min = input
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    if max - min > f32::EPSILON {
        for i in 0..network.input.len() {
            network.input[i] = (network.input[i] - min) / (max - min)
        }
    }
}
pub const NORMALIZATION: NormalizationFn = NormalizationFn {
    function: &normalization,
    description: "",
    formula: "",
};
