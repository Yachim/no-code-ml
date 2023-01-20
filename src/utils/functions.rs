use std::f32::consts;

pub fn sigmoid(n: f32) -> f32 {
    return 1.0 / (1.0 + consts::E.powf(-n));
}

pub fn sigmoid_deriv(n: f32) -> f32 {
    return sigmoid(n)*(1.0 - sigmoid(n));
}

pub fn relu(n: f32) -> f32 {
    if n > 0.0 {
        return n;
    }
    else {
        return 0.0;
    }
}

pub fn relu_deriv(n: f32) -> f32 {
    if n > 0.0 {
        return 1.0;
    }
    else {
        return 0.0;
    }
}
