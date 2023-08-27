use mlrs::{matrix::Matrix, nn::NN};

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let nn = NN::new(vec![2, 2, 1]);
}
