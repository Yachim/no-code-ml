use crate::utils::functions::cost::CostFunc;
use crate::utils::math::dot_product;
use rand::Rng;

pub struct Neuron<'a> {
    /// value before activation function
    pub value: f32,
    /// value after activation function
    pub activation_value: f32,
    pub bias: f32,

    /// connections to the previous layer
    pub weights_prev: Vec<f32>,
    pub neurons_prev: Vec<&'a Self>,

    pub activation_func: &'a dyn Fn(f32) -> f32,
    pub activation_func_derivative: &'a dyn Fn(f32) -> f32,

    cost_func_derivative: CostFunc<'a>,
}

impl<'a> Neuron<'a> {
    pub fn new(
        activation_func: &'a dyn Fn(f32) -> f32,
        activation_func_derivative: &'a dyn Fn(f32) -> f32,
        cost_func_derivative: CostFunc<'a>,
        prev_layer_neuron_cnt: i32,
    ) -> Self {
        let mut rng = rand::thread_rng();

        let weights_prev = (0..prev_layer_neuron_cnt).map(|_| rng.gen()).collect();

        Self {
            value: 0.0,
            activation_value: 0.0,

            weights_prev,
            neurons_prev: Vec::new(), // TODO
            bias: rng.gen(),

            activation_func,
            activation_func_derivative,
            cost_func_derivative,
        }
    }

    pub fn forwardprop(&mut self) {
        let previous_neurons: Vec<f32> = self
            .neurons_prev
            .iter()
            .map(|x| x.activation_value)
            .collect();

        self.value = dot_product(&self.weights_prev, &previous_neurons) + self.bias;

        self.activation_value = (self.activation_func)(self.value);
    }

    pub fn update_values(&mut self, dws: Vec<f32>, db: f32) {
        assert_eq!(self.weights_prev.len(), dws.len());

        for (i, dw) in dws.iter().enumerate() {
            self.weights_prev[i] += dw;
        }

        self.bias += db;
    }
}
