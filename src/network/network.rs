use crate::network::layer::Layer;

pub struct Network<'a> {
    input: Layer<'a>,
    hidden_layers: Layer<'a>,
    output: Layer<'a>,
}

impl Network<'_> {}
