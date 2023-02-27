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

/// (stable) softmax activation function
/// https://eli.thegreenplace.net/2016/the-softmax-function-and-its-derivative/
fn softmax(zs: &Vec<f32>) -> Vec<f32> {
    let max = zs.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let exps = zs.iter().map(|zj| consts::E.powf(*zj - max));
    let sum: f32 = exps.to_owned().sum();
    exps.map(|e| e / sum).collect()
}

/// derivative of the softmax activation function
///
fn sofmax_deriv(zs: &Vec<f32>) -> Vec<f32> {
    vec![0.0]
}
pub const SOFTMAX: ActivationFunction = ActivationFunction {
    //wip, do not use
    function: &softmax,
    derivative: &sofmax_deriv,

    description: "",

    formula: "",
    formula_derivative: "",

    init_fn: &XAVIER,
};

#[cfg(test)]
mod tests {
    use super::softmax;

    #[test]
    fn test_softmax() {
        let res = softmax(&vec![8.0, 5.0, 0.0]);
        let expected = vec![0.9523, 0.0474, 0.0003];
        assert!(res
            .iter()
            .zip(expected)
            .all(|(val, expect)| (val - expect).abs() < 0.001));
    }
}
