use mlrs::nn::NN;

fn main() {
    let nn = NN::new(vec![2, 2, 1]);
    println!("{}", &nn);
}
