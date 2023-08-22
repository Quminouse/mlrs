use crate::matrix::Matrix;

impl Matrix<f32> {
    pub fn sigmoid(&mut self) {
        self.iter_mut().for_each(|x| {
            *x = sigmoid(*x);
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
    fn new(prev: usize, current: usize) -> Self {
        let mut weights = Matrix::new(current, prev);
        let mut biases = Matrix::new(current, 1);
        weights.fill(0.0);
        biases.fill(0.0);

        Layer { weights, biases }
    }
}

impl std::fmt::Display for Layer {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            println!("Weights:{}", self.weights);
            println!("Biases:{}", self.biases);
        Ok(())
    }
}

#[derive(Debug)]
pub struct NN {
    data: Vec<Layer>,
    sizes: Vec<usize>,
}

impl NN {
    pub fn new(sizes: Vec<usize>) -> NN {
        let mut data = Vec::with_capacity(sizes.len());
        let mut p = sizes.iter().peekable();
        while let Some(n) = p.next() {
            match p.peek() {
                Some(m) => {
                    data.push(Layer::new(*n, **m));
                }
                None => {
                }
            }
        }
        NN { data, sizes }
    }
}

impl std::fmt::Display for NN {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.iter().enumerate().for_each(|(x, y)| {
            print!("{}:\n{}", x, y);
        });
        Ok(())
    }
}
