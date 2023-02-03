// https://www.baeldung.com/cs/normalizing-inputs-artificial-neural-network
use crate::network::Network;

pub type NormalizationFn<'a> = &'a dyn Fn(&mut Network);

/// mutates the network so that the input is normalized
pub fn normalization(network: &mut Network) {
    let input = network.input.to_owned();

    let max = input
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
        + f32::EPSILON;

    let min = input
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    for i in 0..network.input.len() {
        network.input[i] = (network.input[i] - min) / (max - min)
    }
}
