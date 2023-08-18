mod matrix;
use matrix::Matrix;

fn main() {
    let mut matrix_1: Matrix<f32> = Matrix::alloc(2, 2);
    let mut matrix_2: Matrix<f32> = Matrix::alloc(2, 2);

    matrix_1.rand();
    matrix_2.rand();

    dbg!(matrix_1 + matrix_2);
}
