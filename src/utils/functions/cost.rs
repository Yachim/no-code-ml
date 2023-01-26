use crate::network::Network;

pub type CostFunc<'a> = &'a dyn Fn(Network, Vec<f32>) -> (Vec<f32>, f32);

/// Derivative of the Mean Squared Error - (a - y)^2;
///  - network: the Network instance to apply on
///  - expected: expected output values; same length as last layer of network
pub fn mse_deriv(network: Network, expected: Vec<f32>) -> (Vec<f32>, f32) {
    assert_eq!(
        network.layers[network.layers.len() - 1].len(),
        expected.len()
    );

    // partial derivative
    // dC/dw = dz/dw × da/dz × dC/da
    // dC/db = dz/db × da/dz × dC/da
    // dC/da_ = dz/da_ × da/dz × dC/da
    //
    // C...cost function
    // w...weight
    // b...bias
    // z...value of neuron before going through activation function (z = a_ × w + b)
    // a...activation function
    // a_...activation of the previous layer
    // y...expected value
    //
    // root (local gradient) = da/d0 × dC/da = a'(O) × 2 × (a - y)
    // dz/db = 1
    // dz/dw = a_
    // dz/da_ =

    (vec![], 0.0) // for testable code
}
