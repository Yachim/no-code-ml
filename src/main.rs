mod utils;
use utils::functions;

fn main() {
    println!("{}", functions::sigmoid(1.0));
    println!("{}", functions::sigmoid_deriv(1.0));
}
