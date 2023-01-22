use itertools::izip;

pub type CostFunc<'a> = &'a dyn Fn(
    Vec<f32>,
    Vec<f32>,
    Vec<f32>,
    Vec<Vec<f32>>,
    Vec<Vec<f32>>,
    &dyn Fn(f32) -> f32,
) -> (Vec<f32>, f32);

/// Derivative of the Mean Squared Error - (a - y)^2; where `a` is the activation and `y` the desired output - with respect to weights and bias
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
pub fn mse_deriv(
    values: Vec<f32>,
    activation_values: Vec<f32>,
    expected_values: Vec<f32>,
    weights: Vec<Vec<f32>>,
    prev_layer_activations: Vec<Vec<f32>>,
    activation_func_derivative: &dyn Fn(f32) -> f32,
) -> (Vec<f32>, f32) {
    assert_eq!(values.len(), activation_values.len());
    assert_eq!(values.len(), expected_values.len());
    assert_eq!(values.len(), weights.len());
    assert_eq!(values.len(), prev_layer_activations.len());

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
    // root = da/d0 × dC/da = a'(O) × 2 × (a - y)
    // dO/db = 1
    // d0/dw = a_

    let batch_size = values.len();
    let ws_cnt = weights[0].len();

    let mut bias_sum = 0.0;
    let mut weights_sum = vec![0.0; ws_cnt];
    for (value, activation, expected, ws, prev_layer_as) in izip!(
        values,
        activation_values,
        expected_values,
        weights,
        prev_layer_activations,
    ) {
        assert_eq!(ws_cnt, ws.len());
        let root = (activation_func_derivative)(value) * 2.0 * (activation - expected);

        bias_sum += root;

        for (i, prev_layer_activation) in prev_layer_as.iter().enumerate() {
            weights_sum[i] += prev_layer_activation * root;
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
