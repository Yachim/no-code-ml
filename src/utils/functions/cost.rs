use crate::network::Network;
use std::iter::zip;

pub type CostFunc<'a> = &'a dyn Fn(&Network, Vec<f32>) -> (Vec<f32>, f32);

/// Derivative of the Mean Squared Error - (a - y)^2;
///  - network: the Network instance to apply on
///  - expected: expected output values; same length as last layer of network
pub fn mse_deriv(network: &Network, expected: Vec<f32>) -> (Vec<f32>, f32) {
    assert_eq!(
        network.activated_layers[network.activated_layers.len() - 1].len(),
        expected.len()
    );

    // the derivatives of cost function
    // with respect to activations from the last layer
    let mut das: Vec<f32> = vec![];
    for (a, y) in zip(
        &network.activated_layers[network.activated_layers.len() - 1],
        &expected,
    ) {
        das.push(2.0 * (a - y));
    }

    for layer_i in (0..(network.layers.len() - 2)).rev() {}

    (vec![], 0.0) // for testable code
}
