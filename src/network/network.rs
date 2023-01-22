use crate::network::layer::Layer;

pub struct Network<'a> {
    input: Layer<'a>,
    hidden_layers: Layer<'a>,
    output: Layer<'a>,
    learning_rate: f32,
}

impl Network<'_> {}
