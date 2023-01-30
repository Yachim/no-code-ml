use crate::network::Network;

pub type CostFunc<'a> = &'a dyn Fn(&Network, Vec<f32>) -> (Vec<Vec<Vec<f32>>>, Vec<Vec<f32>>);

/// Derivative of the Mean Squared Error - (a - y)^2;
///  - network: the Network instance to apply on
///  - expected: expected output values; same length as last layer of network
///
/// returns a change in weights and change in biases
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

            for k in 0..prev_layer.len() {
                let dw =
                    da * network.activations_derivatives[l](network.layers[l][j]) * prev_layer[k];

                neuron_ws.push(dw);
            }
            layer_ws.push(neuron_ws);
        }

        dws.insert(0, layer_ws);
        dbs.insert(0, layer_bs);

        das = new_das;
    }

    return (dws, dbs);
}
