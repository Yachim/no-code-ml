use crate::functions::weight_init::{InitFn, HE, XAVIER};
use std::f32::consts;

pub struct ActivationFunction<'a> {
    pub function: &'a dyn Fn(&Vec<f32>) -> Vec<f32>,
    pub derivative: &'a dyn Fn(&Vec<f32>) -> Vec<f32>,

    pub description: &'a str,

    /// latex formula
    pub formula: &'a str,
    /// latex formula
    pub formula_derivative: &'a str,

    pub init_fn: &'a InitFn<'a>,
}

/// Sigmoid activation function
/// https://en.wikipedia.org/wiki/Sigmoid_function
fn sigmoid(zs: &Vec<f32>) -> Vec<f32> {
    zs.iter()
        .map(|z| (1.0 / (1.0 + consts::E.powf(-z))))
        .collect()
}

/// Derivative of the sigmoid activation function
fn sigmoid_deriv(zs: &Vec<f32>) -> Vec<f32> {
    let sig = sigmoid(zs);
    sig.iter().map(|z| z * (1.0 - z)).collect()
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
fn relu(zs: &Vec<f32>) -> Vec<f32> {
    zs.iter().map(|z| if *z > 0.0 { *z } else { 0.0 }).collect()
}

/// Derivative of the ReLU activation function
fn relu_deriv(zs: &Vec<f32>) -> Vec<f32> {
    zs.iter()
        .map(|z| if *z > 0.0 { 1.0 } else { 0.0 })
        .collect()
}
pub const RELU: ActivationFunction = ActivationFunction {
    function: &relu,
    derivative: &relu_deriv,

    description: "",

    formula: "",
    formula_derivative: "",

    init_fn: &HE,
};
