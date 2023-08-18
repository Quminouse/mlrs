mod matrix;
use matrix::Matrix;

fn main() {
    let mut matrix_1 = Matrix::alloc(2, 2);
    let mut matrix_2 = Matrix::alloc(2, 2);
    matrix_1.fill(3.0);
    matrix_2.fill(2.0);

    dbg!(matrix_1 * matrix_2);
}
