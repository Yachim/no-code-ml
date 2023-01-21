use crate::network::neuron::Neuron;
use std::iter::zip;

/// Derivative of the Mean Squared Error - (a - y)^2; where `a` is the activation and `y` the desired output - with respect to weight and bias
/// activations: the result of the same neuron across different runs
/// expected: the expected result in the same neuron across different runs
///
/// returns: tuple - first element is change in weights, second is change in bias
pub fn mse_deriv(activations: Vec<Neuron>, expected: Vec<f32>) -> (Vec<f32>, f32) {
    assert_eq!(activations.len(), expected.len());
    if activations[0].weights_prev.is_none() || activations[0].neurons_prev.is_none() {
        panic!("Neuron does not have previous layer")
    }

    // partial derivative
    // dC/dw = dO/dw × da/dO × dC/da
    // dC/db = dO/db × da/dO × dC/da
    //
    //
    // C...cost function
    // w...weight
    // b...bias
    // O...value of neuron before going through activation function (O = a_ × w + b)
    // a...activation function
    // a_...activation of the previous layer
    // y...expected value
    //
    // root = da/d0 × dC/da = a'(O) × 2 × (a - y)
    // dO/db = 1
    // d0/dw = a_

    let batch_size = activations[0].weights_prev.as_ref().unwrap().len();

    let mut bias_sum = 0.0;
    let mut weights_sum = vec![0.0; batch_size];
    for (neuron, y) in zip(activations, expected) {
        let root =
            (neuron.activation_func_derivative)(neuron.value) * 2.0 * (neuron.activation_value - y);
        bias_sum += root;

        for (i, prev_layer_neuron) in neuron.neurons_prev.unwrap().iter().enumerate() {
            weights_sum[i] += prev_layer_neuron.activation_value * root;
        }
    }

    (
        weights_sum
            .iter()
            .map(|x| x / (batch_size as f32))
            .collect(),
        bias_sum / (batch_size as f32),
    )
}
