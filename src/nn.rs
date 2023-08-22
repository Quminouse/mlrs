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
