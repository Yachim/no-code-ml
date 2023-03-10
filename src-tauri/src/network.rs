use crate::functions::{
    activation::ActivationFunction, cost::CostFunc, input_normalizations::NormalizationFn,
};
use crate::utils::math::dot_product;
use chrono::offset::Local;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::iter::zip;

pub type TrainingData = Vec<(Vec<f32>, Vec<f32>)>;

/// L = number of layers (except the first (input) layer)
/// l = current layer
/// N = number of neurons in the lth layer
/// n = current neuron from the layer N
/// M = number of neurons in the (l - 1)th layer
/// m = current neuron from the layer M
pub struct Network<'a> {
    /// the input fields
    pub input: Vec<f32>,

    /// outer vector has len of L
    /// each element represents a layer (l)
    /// does not inluclude the first (input) layer
    ///
    /// inner vectors can have different len of N
    /// each element represents a neuron's (n) value before activation
    pub layers: Vec<Vec<f32>>,

    /// outer vector has len of L
    /// each element represents a layer (l)
    /// does not inluclude the first (input) layer
    ///
    /// inner vectors can have different len of N
    /// each element represents a neuron's (n) value after activation
    pub activated_layers: Vec<Vec<f32>>,

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
    pub weights: Vec<Vec<Vec<f32>>>,

    /// outer vector has len of L
    /// each element represents a layer (l)
    /// does not inluclude the first (input) layer
    ///
    /// inner vectors can have different len of N
    /// each element represents a bias of a neuron (n)
    biases: Vec<Vec<f32>>,

    /// vector has len of (L - 1)
    /// contains activation functions for all layers except the first (input) layer
    /// does not inluclude the first (input) layer
    pub activation_functions: Vec<&'a ActivationFunction<'a>>,

    cost_func: &'a CostFunc<'a>,

    /// log epochs when training?
    pub log_epochs: bool,

    /// log costs at the beginning and at the end of training?
    pub log_costs: bool,

    normalization_fn: &'a NormalizationFn<'a>,
}

impl<'a> Network<'a> {
    /// Creates a new network
    ///  - training_cnt: number of neurons in the input layer
    ///  - hidden_layers: vector of usize; each usize represents the number of neurons in a layer,
    ///  - out_labels: vector of strings; each string represents a label for the output,
    ///  - activation_functions: activation functions for each layer, including output
    ///  - cost_func: cost function
    pub fn new(
        training_cnt: usize,
        hidden_layers: Vec<usize>,
        out_labels: Vec<&'a str>,
        activation_functions: Vec<&'a ActivationFunction>,
        cost_func: &'a CostFunc,
        normalization_fn: &'a NormalizationFn,
    ) -> Self {
        assert_eq!(activation_functions.len(), hidden_layers.len() + 1);

        // all layers except the input (hidden + output)
        // contains neurons count
        let ls: Vec<usize> = hidden_layers
            .iter()
            .chain(&[out_labels.len()])
            .copied()
            .collect();

        let input = vec![0.0; training_cnt];

        let mut layers: Vec<Vec<f32>> = vec![];
        let mut activated_layers: Vec<Vec<f32>> = vec![];

        for cnt in ls {
            layers.push(vec![0.0; cnt]);
            activated_layers.push(vec![0.0; cnt]);
        }

        let mut s = Self {
            input,
            layers,
            activated_layers,
            weights: vec![],
            biases: vec![],
            activation_functions,
            cost_func,
            out_labels,
            log_epochs: false,
            log_costs: false,
            normalization_fn,
        };

        s.initialize_params();

        s
    }

    /// randomly sets weights and biases
    fn initialize_params(&mut self) {
        let mut biases: Vec<Vec<f32>> = vec![];
        let mut weights: Vec<Vec<Vec<f32>>> = vec![];

        for (i, layer) in self.layers.iter().enumerate() {
            let layer_biases: Vec<f32> = vec![0.0; layer.len()];
            let mut layer_weights: Vec<Vec<f32>> = vec![];

            let layer_weights_init_fn = self.activation_functions[i].init_fn.function;

            for _ in 0..layer.len() {
                let mut neuron_weights: Vec<f32> = vec![];
                // count of neurons in prevous layer
                let neuron_cnt = if i == 0 {
                    self.input.len()
                } else {
                    self.layers[i - 1].len()
                };

                for _ in 0..neuron_cnt {
                    neuron_weights.push(layer_weights_init_fn(neuron_cnt));
                }

                layer_weights.push(neuron_weights);
            }

            biases.push(layer_biases);
            weights.push(layer_weights);
        }

        self.weights = weights;
        self.biases = biases;
    }

    /// feedforwards (sets values) to the layer (in layers and activated_layers) with the given index
    fn feedforward_layer(&mut self, layer_index: usize) {
        let prev_layer: Vec<f32> = if layer_index == 0 {
            self.input.to_owned()
        } else {
            self.activated_layers[layer_index - 1].to_owned()
        };

        for i in 0..self.layers[layer_index].len() {
            let weights = &self.weights[layer_index][i];
            let bias = self.biases[layer_index][i];

            self.layers[layer_index][i] = dot_product(&prev_layer, weights) + bias;
        }
        self.activated_layers[layer_index] =
            (self.activation_functions[layer_index].function)(&self.layers[layer_index]);
    }

    fn feedforward(&mut self) {
        for i in 0..self.layers.len() {
            self.feedforward_layer(i);
        }
    }

    // returns change in weights and biases
    fn backprop(&self, expected: &Vec<f32>) -> (Vec<Vec<Vec<f32>>>, Vec<Vec<f32>>) {
        // prev is used as in the previous iteration (l + 1 layer, since the range is backwards)
        // all throughout this function

        // dC/da(l + 1) for any layer l in the network
        let mut prev_layer_cost_derivatives: Vec<f32> = vec![];

        // dC/da(L) where L is the last layer
        let last_layer_cost_derivatives =
            (self.cost_func.derivative)(&self.activated_layers.last().unwrap(), expected);

        let mut dws: Vec<Vec<Vec<f32>>> = vec![];
        let mut dbs: Vec<Vec<f32>> = vec![];

        for l in (0..self.activated_layers.len()).rev() {
            let mut layer_dws: Vec<Vec<f32>> = vec![];
            let mut layer_dbs: Vec<f32> = vec![];

            let is_last_layer = l == self.activated_layers.len() - 1;

            // da(l)/dz(l)
            let layer_activation_derivatives =
                (self.activation_functions[l].derivative)(&self.layers[l]);

            // da(l + 1)/dz(l + 1)
            // used only if not last layer
            let prev_layer_activation_derivatives: Vec<f32> = if is_last_layer {
                vec![]
            } else {
                (self.activation_functions[l + 1].derivative)(&self.layers[l + 1])
            };

            // dC/da(l)
            let mut layer_cost_derivatives: Vec<f32> = vec![];

            for j in 0..self.activated_layers[l].len() {
                // dC/da(l, j)
                let neuron_cost_derivative: f32 = if is_last_layer {
                    last_layer_cost_derivatives[j]
                } else {
                    (0..self.activated_layers[l + 1].len())
                        .map(|i| {
                            prev_layer_cost_derivatives[i]
                                * prev_layer_activation_derivatives[i]
                                * self.weights[l + 1][i][j]
                        })
                        .sum()
                };
                layer_cost_derivatives.push(neuron_cost_derivative);

                let mut neuron_dws: Vec<f32> = vec![];

                for k in 0..self.weights[l][j].len() {
                    // next as in next in iteration
                    // previous when doing feedforward
                    let next_layer_neuron_activation = if l == 0 {
                        self.input[k]
                    } else {
                        self.activated_layers[l - 1][k]
                    };

                    neuron_dws.push(
                        neuron_cost_derivative
                            * layer_activation_derivatives[j]
                            * next_layer_neuron_activation,
                    );
                }

                layer_dbs.push(neuron_cost_derivative * layer_activation_derivatives[j]);
                layer_dws.push(neuron_dws);
            }

            dws.insert(0, layer_dws);
            dbs.insert(0, layer_dbs);

            prev_layer_cost_derivatives = layer_cost_derivatives;
        }

        (dws, dbs)
    }

    /// does gradient descent over a mini batch
    fn gradient_descent(&mut self, batch: &TrainingData, learning_rate: f32) {
        let batch_size = batch.len();

        let mut total_dws: Vec<Vec<Vec<f32>>> = vec![];
        let mut total_dbs: Vec<Vec<f32>> = vec![];

        for l in 0..self.weights.len() {
            let mut layer_dws: Vec<Vec<f32>> = vec![];
            let layer_dbs: Vec<f32> = vec![0.0; self.weights[l].len()];

            for j in 0..self.weights[l].len() {
                let neuron_dws = vec![0.0; self.weights[l][j].len()];

                layer_dws.push(neuron_dws);
            }

            total_dws.push(layer_dws);
            total_dbs.push(layer_dbs);
        }

        for (input, expected) in batch.iter() {
            assert_eq!(input.len(), self.input.len());

            self.predict(input.to_vec());

            let (dws, dbs) = self.backprop(&expected.to_vec());

            for l in 0..dws.len() {
                for j in 0..dws[l].len() {
                    for k in 0..dws[l][j].len() {
                        total_dws[l][j][k] += dws[l][j][k];
                        total_dbs[l][j] += dbs[l][j];
                    }
                }
            }
        }

        for l in 0..total_dws.len() {
            for j in 0..total_dws[l].len() {
                for k in 0..total_dws[l][j].len() {
                    self.weights[l][j][k] -=
                        total_dws[l][j][k] / (batch_size as f32) * learning_rate;
                }

                self.biases[l][j] -= total_dbs[l][j] / (batch_size as f32) * learning_rate;
            }
        }
    }

    /// splits training data into mini batches
    fn batch_gradient_descent(
        &mut self,
        training_data: &TrainingData,
        learning_rate: f32,
        batch_size: usize,
    ) {
        for batch_start_index in (0..training_data.len()).step_by(batch_size) {
            let batch = training_data[batch_start_index..batch_start_index + batch_size].to_owned();
            self.gradient_descent(&batch, learning_rate);
        }
    }

    pub fn train(
        &mut self,
        training_data: TrainingData,
        iteration_cnt: usize,
        learning_rate: f32,
        batch_size: usize,
    ) {
        let mut rng = rand::thread_rng();

        let time_start = Local::now();
        println!("beginning training at {time_start}");

        let mut training_data_mut = training_data;

        if self.log_costs {
            println!("average cost: {}", self.average_cost(&training_data_mut));
        }

        for i in 0..iteration_cnt {
            if self.log_epochs {
                let epoch = i + 1;
                let time_epoch = Local::now();

                println!("beginning training epoch {epoch} out of {iteration_cnt} at {time_epoch}");
            }

            training_data_mut.shuffle(&mut rng);
            self.batch_gradient_descent(&training_data_mut, learning_rate, batch_size);
        }

        if self.log_costs {
            println!("average cost: {}", self.average_cost(&training_data_mut));
        }

        let time_end = Local::now();
        println!("finishing training at {time_end}");
    }

    /// predicts the output from the given data
    pub fn predict(&mut self, data: Vec<f32>) {
        self.input = data;
        (self.normalization_fn.function)(self);

        self.feedforward();
    }

    /// returns a hasmap with keys from out_labels and their corresponding values
    pub fn get_output(&self) -> HashMap<&str, f32> {
        let mut map: HashMap<&str, f32> = HashMap::new();
        for (label, val) in zip(
            &self.out_labels,
            &self.activated_layers[self.activated_layers.len() - 1],
        ) {
            map.insert(*label, *val);
        }

        map
    }

    /// returns a key and corresponding value with the highest value in the output
    pub fn get_best_output(&self) -> (&str, f32) {
        let val = zip(
            &self.out_labels,
            &self.activated_layers[self.activated_layers.len() - 1],
        )
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .unwrap();

        (*val.0, *val.1)
    }

    /// predicts values
    /// computes the average cost against the training data
    pub fn average_cost(&mut self, data: &TrainingData) -> f32 {
        let mut cost_sum = 0.0;

        for (inputs, expected) in data {
            self.predict(inputs.to_vec());
            cost_sum += (self.cost_func.function)(
                self.activated_layers.last().unwrap(),
                &expected.to_vec(),
            );
        }

        cost_sum / (data.len() as f32)
    }

    pub fn debug(&self) {
        println!("inputs: {:?}", self.input);
        println!("layers: {:?}", self.layers);
        println!("activations: {:?}", self.activated_layers);
        println!("weights: {:?}", self.weights);
        println!("biases: {:?}", self.biases);
    }
}

#[cfg(test)]
mod tests {
    use super::Network;
    use crate::functions::{
        activation::SIGMOID, cost::MSE, input_normalizations::NO_NORMALIZATION,
    };
    use std::collections::HashMap;

    #[test]
    fn test_ws_cnt() {
        let net = Network::new(
            3,
            vec![2, 3],
            vec!["1", "2"],
            vec![&SIGMOID, &SIGMOID, &SIGMOID],
            &MSE,
            &NO_NORMALIZATION,
        );

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

    #[test]
    fn test_bias_cnt() {
        let net = Network::new(
            3,
            vec![2, 3],
            vec!["1", "2"],
            vec![&SIGMOID, &SIGMOID, &SIGMOID],
            &MSE,
            &NO_NORMALIZATION,
        );

        let mut total_biases = 0;
        for i in net.biases {
            for _ in i {
                total_biases += 1;
            }
        }

        assert_eq!(total_biases, 7);
    }

    #[test]
    fn test_neurons_cnt() {
        let net = Network::new(
            3,
            vec![2, 3],
            vec!["1", "2"],
            vec![&SIGMOID, &SIGMOID, &SIGMOID],
            &MSE,
            &NO_NORMALIZATION,
        );

        let mut total_neurons = 0;
        for i in net.layers {
            for _ in i {
                total_neurons += 1;
            }
        }
        total_neurons += net.input.len();

        assert_eq!(total_neurons, 10);
    }

    #[test]
    fn test_feedforward_layer() {
        let mut net = Network::new(
            3,
            vec![],
            vec!["1", "2", "3"],
            vec![&SIGMOID],
            &MSE,
            &NO_NORMALIZATION,
        );

        net.input = vec![3.0, 2.0];
        net.weights[0][0] = vec![3.0, 4.0];
        net.weights[0][1] = vec![2.0, 4.0];
        net.weights[0][2] = vec![3.0, 5.0];
        net.biases[0] = vec![1.0, 1.0, 3.0];

        net.feedforward_layer(0);

        assert_eq!(net.layers[0][0], 18.0);
        assert_eq!(net.layers[0][1], 15.0);
        assert_eq!(net.layers[0][2], 22.0);
    }

    #[test]
    fn test_best_output() {
        let mut net = Network::new(
            3,
            vec![],
            vec!["1", "2", "3"],
            vec![&SIGMOID],
            &MSE,
            &NO_NORMALIZATION,
        );

        net.activated_layers[0] = vec![3.0, 2.0, 1.0];
        let out = net.get_best_output();
        assert_eq!(out.1, 3.0);
        assert_eq!(out.0, "1");
    }

    #[test]
    fn test_outputs() {
        let mut net = Network::new(
            3,
            vec![],
            vec!["1", "2", "3"],
            vec![&SIGMOID],
            &MSE,
            &NO_NORMALIZATION,
        );

        net.activated_layers[0] = vec![3.0, 2.0, 1.0];

        let out = net.get_output();
        let expected_map: HashMap<&str, f32> = HashMap::from([("1", 3.0), ("2", 2.0), ("3", 1.0)]);

        assert_eq!(out.len(), expected_map.len());
        assert!(out.keys().all(|key| expected_map.contains_key(key)
            && f32::abs(out.get(key).unwrap() - expected_map.get(key).unwrap()) < f32::EPSILON));
    }
}
