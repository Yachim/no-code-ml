use crate::utils::functions::weight_init::{InitFn, HE, XAVIER};
use std::f32::consts;

pub struct ActivationFunction<'a> {
    pub function: &'a dyn Fn(f32) -> f32,
    pub derivative: &'a dyn Fn(f32) -> f32,

    pub description: &'a str,

    /// latex formula
    pub formula: &'a str,
    /// latex formula
    pub formula_derivative: &'a str,

    pub init_fn: &'a InitFn<'a>,
}

/// Sigmoid activation function
/// https://en.wikipedia.org/wiki/Sigmoid_function
fn sigmoid(n: f32) -> f32 {
    1.0 / (1.0 + consts::E.powf(-n))
}

/// Derivative of the sigmoid activation function
fn sigmoid_deriv(n: f32) -> f32 {
    let sig_n = sigmoid(n);
    sig_n * (1.0 - sig_n)
}
pub const SIGMOID: ActivationFunction = ActivationFunction {
    function: &sigmoid,
    derivative: &sigmoid_deriv,

    description: "",

    formula: "",
    formula_derivative: "",

    init_fn: &XAVIER,
};

/// ReLU activation function
/// https://en.wikipedia.org/wiki/Rectifier_(neural_networks)
fn relu(n: f32) -> f32 {
    if n > 0.0 {
        n
    } else {
        0.0
    }
}

/// Derivative of the ReLU activation function
fn relu_deriv(n: f32) -> f32 {
    if n > 0.0 {
        1.0
    } else {
        0.0
    }
}
pub const RELU: ActivationFunction = ActivationFunction {
    function: &relu,
    derivative: &relu_deriv,

    description: "",

    formula: "",
    formula_derivative: "",

    init_fn: &HE,
};
