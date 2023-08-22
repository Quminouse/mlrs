use mlrs::matrix::Matrix;

fn main() {
    let mut test_mat: Matrix<f32> = Matrix::alloc(3, 2);
    test_mat.rand();
    test_mat.sigmoid();
    dbg!(test_mat);
}
