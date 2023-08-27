use crate::matrix::Matrix;
use rand;

impl Matrix<f32> {
    pub fn sigmoid(&mut self) {
        self.iter_mut().for_each(|x| {
            *x = sigmoid(*x);
        });
    }
    pub fn rand(&mut self) {
        self.iter_mut().for_each(|x| {
            *x = rand::random();
        });
    }
}

fn sigmoid(i: f32) -> f32 {
    1.0 / (1.0 + std::f32::consts::E.powf(-i))
}

#[derive(Debug)]
struct Layer {
    weights: Matrix<f32>,
    biases: Matrix<f32>,
}

impl Layer {
    pub fn new(prev: usize, current: usize) -> Self {
        Layer {
            weights: Matrix::new(current, prev),
            biases: Matrix::new(current, 1),
        }
    }
    fn activate(&self, activation: Matrix<f32>) -> Matrix<f32> {
        return (self.weights.clone() * activation.clone()) + self.biases.clone();
    }
}

impl std::fmt::Display for Layer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Weights:\n {}\n", self.weights)?;
        write!(f, "Biases: \n {}", self.biases)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct NN {
    data: Vec<Layer>,
}

impl NN {
    pub fn new(sizes: Vec<usize>) -> NN {
        let mut data = Vec::with_capacity(sizes.len());
        let mut iter = sizes.iter().peekable();
        while let Some(n) = iter.next() {
            match iter.peek() {
                Some(m) => {
                    data.push(Layer::new(*n, **m));
                }
                None => {}
            }
        }
        NN { data }
    }
    pub fn activate(&self, mut input: Matrix<f32>) -> Matrix<f32> {
        for i in self.data.iter() {
            input = i.activate(input);
        }
        return input;
    }
    pub fn fill_weights(&mut self, value: f32) {
        self.data.iter_mut().for_each(|x| {
            x.weights.fill(value);
        });
    }
    pub fn fill_biases(&mut self, value: f32) {
        self.data.iter_mut().for_each(|x| {
            x.biases.fill(value);
        });
    }
}

impl std::fmt::Display for NN {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.iter().for_each(|x| {
            writeln!(f, "\n{}", x).unwrap();
        });
        Ok(())
    }
}
