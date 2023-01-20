mod utils;
mod network;
use utils::functions;

fn main() {
    println!("{}", functions::sigmoid(1.0));
    println!("{}", functions::sigmoid_deriv(1.0));

    println!("{}", functions::relu(42.0));
    println!("{}", functions::relu_deriv(42.0));
    println!("{}", functions::relu(-42.0));
    println!("{}", functions::relu_deriv(-42.0));
}
