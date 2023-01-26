use crate::network::Network;

pub type CostFunc<'a> = &'a dyn Fn(Network, Vec<f32>) -> (Vec<f32>, f32);

/// Derivative of the Mean Squared Error - (a - y)^2;
/// where `a` is the activation and `y` the desired output - with respect to weights and bias
/// the mean has to be calculated outside - summed and divided
///  - values: the value before going through activation function of the same neuron across different runs
///  - activation_values: the activations value of the same neuron across different runs
///  - expected: the expected result in the same neuron across different runs
///  - weights:
///    - inner vector: weights connecting neuron to all neurons from previous layers
///    - outer vector: different runs, same neuron's weights
///  - prev_layer_activations:
///    - inner vector:
///    - outer vector:
///  - activation_func_derivative: derivative of the activation function
///
/// returns: tuple - first element is change in weights, second is change in bias
pub fn mse_deriv(network: Network, expected_values: Vec<f32>) -> (Vec<f32>, f32) {
    // partial derivative
    // dC/dw = dO/dw × da/dO × dC/da
    // dC/db = dO/db × da/dO × dC/da
    //
    // C...cost function
    // w...weight
    // b...bias
    // O...value of neuron before going through activation function (O = a_ × w + b)
    // a...activation function
    // a_...activation of the previous layer
    // y...expected value
    //
    // root (local gradient) = da/d0 × dC/da = a'(O) × 2 × (a - y)
    // dO/db = 1
    // d0/dw = a_

    (vec![], 0.0) // for testable code
}
