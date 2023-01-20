pub struct Neuron {
    /// value before activation function
    value: f32,
    /// value after activation function
    activation_value: f32,

    /// connections to the previous layer
    weights: Option<Vec<f32>>,
}

impl Neuron {}
