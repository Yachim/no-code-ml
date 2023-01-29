use crate::network::Network;
use std::iter::zip;

pub type CostFunc<'a> = &'a dyn Fn(&mut Network, Vec<f32>) -> (Vec<Vec<Vec<f32>>>, Vec<Vec<f32>>);

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

    // the derivatives of cost function
    // with respect to activations
    let mut das: Vec<f32> = vec![];
    for (a, y) in zip(
        &network.activated_layers[network.activated_layers.len() - 1],
        &expected,
    ) {
        das.push(2.0 * (a - y));
    }

    let mut new_ws: Vec<Vec<Vec<f32>>> = vec![];
    let mut new_bs: Vec<Vec<f32>> = vec![];

    for l in (0..(network.activated_layers.len() - 2)).rev() {
        new_ws.insert(0, vec![]);
        new_bs.insert(0, vec![]);

        let l_plus_one = l + 1;
        let n_l_plus_one = network.activated_layers[l_plus_one].len();
        assert_eq!(n_l_plus_one, das.len());

        let mut new_das: Vec<f32> = vec![];

        let n_l = network.layers[l].len();

        for j in 0..n_l_plus_one {
            new_ws[l].push(vec![]);
            // activations_derivatives[l]?
            new_bs[l].push(das[l] * network.activations_derivatives[l](network.layers[l][j]));

            for k in 0..n_l {
                // TODO: might not be correct
                let prev_activation = if l == 0 {
                    network.input[k]
                } else {
                    network.activated_layers[l - 1][k]
                };

                new_ws[l][j].push(
                    das[l]
                        * network.activations_derivatives[l](network.layers[l][j])
                        * prev_activation,
                )
            }
        }

        for k in 0..n_l {
            let mut new_da = 0.0;
            for j in 0..n_l_plus_one {
                // activations_derivatives[l_plus_one]?
                new_da += das[j]
                    * network.activations_derivatives[l_plus_one](network.layers[l_plus_one][j])
                    * network.weights[l_plus_one][j][k];
            }
            new_das.push(new_da);
        }
    }

    return (new_ws, new_bs);
}
