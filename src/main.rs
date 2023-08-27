use mlrs::{matrix::Matrix, nn::NN};

fn main() {
    let mut nn = NN::new(vec![2, 2, 1]);
    let mut activation = Matrix::new(2, 1);

    nn.fill_weights(0.5);
    nn.fill_biases(1.0);

    activation.fill(1.0);

    println!("{}", nn);

    println!("{}", nn.activate(activation));
}
