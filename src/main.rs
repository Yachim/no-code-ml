mod network;
mod utils;
use utils::math::dot_product;

fn main() {
    let a = vec![1.0, 2.0, 3.0];
    let b = vec![6.0, 5.0, 4.0];

    println!("{}", dot_product(a, b))
}
