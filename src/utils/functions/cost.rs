use crate::network::Network;
use std::iter::zip;

pub type CostFunc<'a> = &'a dyn Fn(&Network, Vec<f32>) -> f32;
pub type CostFuncDeriv<'a> = &'a dyn Fn(&Network, Vec<f32>) -> (Vec<Vec<Vec<f32>>>, Vec<Vec<f32>>);

/// Mean Squared Error
///  - network: the Network instance to apply on
///  - expected: expected output values; same length as last layer of network
///
/// returns the cost
pub fn mse(network: &Network, expected: Vec<f32>) -> f32 {
    let last_layer = &network.activated_layers[network.activated_layers.len() - 1];

    assert_eq!(last_layer.len(), expected.len());

    let sum: f32 = zip(last_layer, &expected)
        .map(|(a, y)| (a - y).powf(2.0))
        .sum();

    sum / (expected.len() as f32)
}

/// Derivative of the Mean Squared Error
///  - network: the Network instance to apply on
///  - expected: expected output values; same length as last layer of network
///
/// returns a change in weights and change in biases (gradient);
/// the opposite of gradient is done outside of the function
pub fn mse_deriv(network: &Network, expected: Vec<f32>) -> (Vec<Vec<Vec<f32>>>, Vec<Vec<f32>>) {
    assert_eq!(
        network.activated_layers[network.activated_layers.len() - 1].len(),
        expected.len()
    );

    // derivatives of cost function with respect to activations from the next layer
    let mut das: Vec<f32> = vec![];

    let mut dws: Vec<Vec<Vec<f32>>> = vec![];
    let mut dbs: Vec<Vec<f32>> = vec![];

    for l in (0..network.activated_layers.len()).rev() {
        let mut new_das: Vec<f32> = vec![];

        let mut layer_ws: Vec<Vec<f32>> = vec![];
        let mut layer_bs: Vec<f32> = vec![];

        for j in 0..network.activated_layers[l].len() {
            let is_last_layer = l == network.activated_layers.len() - 1;

            let da = if is_last_layer {
                2.0 * (network.activated_layers[l][j] - expected[j])
            } else {
                (0..network.activated_layers[l + 1].len())
                    .map(|i| {
                        das[i]
                            * network.activations_derivatives[l + 1](network.layers[l + 1][i])
                            * network.weights[l + 1][i][j]
                    })
                    .sum()
            };
            new_das.push(da);

            let db = da * network.activations_derivatives[l](network.layers[l][j]);
            layer_bs.push(db);

            let mut neuron_ws: Vec<f32> = vec![];
            let prev_layer = if l == 0 {
                &network.input
            } else {
                &network.activated_layers[l - 1]
            };

            for prev_layer_neuron in prev_layer {
                let dw = da
                    * network.activations_derivatives[l](network.layers[l][j])
                    * prev_layer_neuron;

                neuron_ws.push(dw);
            }
            layer_ws.push(neuron_ws);
        }

        dws.insert(0, layer_ws);
        dbs.insert(0, layer_bs);

        das = new_das;
    }

    (dws, dbs)
}

#[cfg(test)]
mod tests {
    use super::{mse, mse_deriv};
    use crate::network::Network;
    use crate::utils::functions::{
        activation::{sigmoid, sigmoid_deriv},
        input_normalizations::normalization,
    };

    #[test]
    fn test_mse() {
        let mut net = Network::new(
            1,
            vec![],
            vec!["1"],
            vec![&sigmoid],
            vec![&sigmoid_deriv],
            &mse_deriv,
            &mse,
            &normalization,
        );

        net.activated_layers[0] = vec![1.0];

        assert_eq!(mse(&net, vec![1.0]), 0.0);
        assert_eq!(mse(&net, vec![0.5]), 0.25);
        assert_eq!(mse(&net, vec![0.0]), 1.0);
    }
}
