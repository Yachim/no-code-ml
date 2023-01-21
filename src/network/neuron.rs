use crate::utils::math::dot_product;
use rand::Rng;

pub struct Neuron<'a> {
    /// value before activation function
    pub value: f32,
    /// value after activation function
    pub activation_value: f32,
    pub bias: f32,

    /// connections to the previous layer
    pub weights_prev: Option<Vec<f32>>,
    pub neurons_prev: Option<Vec<&'a Self>>,

    pub activation_func: &'a dyn Fn(f32) -> f32,
    pub activation_func_derivative: &'a dyn Fn(f32) -> f32,
}

impl<'a> Neuron<'a> {
    pub fn new(
        activation_func: &'a dyn Fn(f32) -> f32,
        activation_func_derivative: &'a dyn Fn(f32) -> f32,
        prev_layer_neuron_cnt: Option<i32>,
    ) -> Self {
        let mut rng = rand::thread_rng();

        let weights_prev = if prev_layer_neuron_cnt.is_some() {
            Some(
                (0..prev_layer_neuron_cnt.unwrap_or(0))
                    .map(|_| rng.gen())
                    .collect(),
            )
        } else {
            None
        };

        Self {
            value: 0.0,
            activation_value: 0.0,

            weights_prev,
            neurons_prev: None, // TODO
            bias: rng.gen(),

            activation_func,
            activation_func_derivative,
        }
    }

    pub fn calculate_values(&mut self) {
        if self.weights_prev.is_none() || self.neurons_prev.is_none() {
            panic!("weights or neurons from previous layer are none")
        }

        let previous_neurons: Vec<f32> = self
            .neurons_prev
            .as_ref()
            .unwrap()
            .into_iter()
            .map(|x| x.activation_value)
            .collect();

        self.value = dot_product(
            self.weights_prev.as_ref().unwrap(),
            previous_neurons.as_ref(),
        ) + self.bias;

        self.activation_value = (self.activation_func)(self.value);
    }
}
