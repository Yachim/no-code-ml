use crate::utils::functions::cost::CostFunc;
use rand::Rng;

/// L = number of layers
/// l = current layer
/// N = number of neurons in the lth layer
/// n = current neuron from the layer N
/// M = number of neurons in the (l - 1)th layer
/// m = current neuron from the layer M
pub struct Network<'a> {
    /// outer vector has len of L
    /// each element represents a layer (l)
    /// first (input) layer is empty
    ///
    /// inner vectors can have different len of N
    /// each element represents a neuron's (n) value before activation
    layers: Vec<Vec<f32>>,

    /// outer vector has len of L
    /// each element represents a layer (l)
    ///
    /// inner vectors can have different len of N
    /// each element represents a neuron's (n) value after activation
    activated_layers: Vec<Vec<f32>>,

    /// same length as the last layer
    /// labels for the outputs
    out_labels: Vec<&'a str>,

    /// outer vector has len of (L - 1)
    /// each element represents all the weights between lth layer (N) and (l - 1)th layer (M)
    ///
    /// middle vectors can have different len of N
    /// each element represents all weights connected to neuron n
    ///
    /// inner vectors can have different len of M
    /// each element represents a weight conected from neuron m to neuron n
    weights: Vec<Vec<Vec<f32>>>,

    /// outer vector has len of L
    /// each element represents a layer (l)
    /// first (input) layer is empty
    ///
    /// inner vectors can have different len of N
    /// each element represents a bias of a neuron (n)
    biases: Vec<Vec<f32>>,

    /// vector has len of (L - 1)
    /// contains activation functions for all layers except the first (input) layer
    activations: Vec<&'a dyn Fn(f32) -> f32>,
    activations_derivatives: Vec<&'a dyn Fn(f32) -> f32>,

    /// derivative of the cost function
    cost_func_derivative: CostFunc<'a>,
}

impl<'a> Network<'a> {
    /// Creates a new network
    ///  - input_cnt: count of input neurons
    ///  - hidden layers: vector of usize; each usize represents the number of neurons in a layer
    ///  - output_labels: vector of strings; each string represents a label for the output
    pub fn new(
        inp_cnt: usize,
        hidden_layers: Vec<usize>,
        out_labels: Vec<&'a str>,
        activations: Vec<&'a dyn Fn(f32) -> f32>,
        activations_derivatives: Vec<&'a dyn Fn(f32) -> f32>,
        cost_func_derivative: CostFunc<'a>,
    ) -> Self {
        assert_eq!(activations_derivatives.len(), activations.len());
        assert_eq!(activations.len(), hidden_layers.len() + 1);
        let mut rng = rand::thread_rng();

        // all layers except the input (hidden + output)
        // contains neurons count
        let ls: Vec<usize> = hidden_layers
            .iter()
            .chain(&[out_labels.len()])
            .map(|&x| x)
            .collect();

        let mut layers: Vec<Vec<f32>> = vec![vec![]];
        let mut activated_layers: Vec<Vec<f32>> = vec![vec![0.0; inp_cnt]];

        let mut biases: Vec<Vec<f32>> = vec![];
        let mut weights: Vec<Vec<Vec<f32>>> = vec![];

        for (i, cnt) in ls.iter().enumerate() {
            layers.push(vec![0.0; *cnt]);
            activated_layers.push(vec![0.0; *cnt]);

            let mut layer_biases: Vec<f32> = vec![];
            let mut layer_weights: Vec<Vec<f32>> = vec![];

            for _ in 0..*cnt {
                layer_biases.push(rng.gen_range(0.0..=1.0));

                let mut neuron_weights: Vec<f32> = vec![];
                let neuron_cnt = if i == 0 { inp_cnt } else { ls[i - 1] };

                for _ in 0..neuron_cnt {
                    neuron_weights.push(rng.gen_range(0.0..=1.0));
                }

                layer_weights.push(neuron_weights);
            }

            biases.push(layer_biases);
            weights.push(layer_weights);
        }

        Self {
            layers,
            activated_layers,
            weights,
            biases,
            activations,
            activations_derivatives,
            cost_func_derivative,
            out_labels,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Network;
    use crate::utils::functions::{
        activation::{sigmoid, sigmoid_deriv},
        cost::mse_deriv,
    };

    fn net() -> Network<'static> {
        Network::new(
            3,
            vec![2, 3],
            vec!["1", "2"],
            vec![&sigmoid, &sigmoid, &sigmoid],
            vec![&sigmoid_deriv, &sigmoid_deriv, &sigmoid_deriv],
            &mse_deriv,
        )
    }

    #[test]
    fn test_ws_cnt() {
        let net = net();

        let mut total_ws = 0;
        for i in net.weights {
            for j in i {
                for _ in j {
                    total_ws += 1;
                }
            }
        }

        assert_eq!(total_ws, 18);
    }
}
