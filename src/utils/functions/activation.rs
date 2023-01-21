use std::f32::consts;

/// Sigmoid activation function
/// https://en.wikipedia.org/wiki/Sigmoid_function
pub fn sigmoid(n: f32) -> f32 {
    1.0 / (1.0 + consts::E.powf(-n))
}

/// Derivative of the sigmoid activation function
pub fn sigmoid_deriv(n: f32) -> f32 {
    sigmoid(n) * (1.0 - sigmoid(n))
}

/// ReLU activation function
/// https://en.wikipedia.org/wiki/Rectifier_(neural_networks)
pub fn relu(n: f32) -> f32 {
    if n > 0.0 {
        n
    } else {
        0.0
    }
}

/// Derivative of the ReLU activation function
pub fn relu_deriv(n: f32) -> f32 {
    if n > 0.0 {
        1.0
    } else {
        0.0
    }
}
