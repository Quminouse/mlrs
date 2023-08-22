use mlrs::matrix::Matrix;

fn main() {
    let mut test_mat: Matrix<f32> = Matrix::alloc(3, 2);
    test_mat.rand();
    dbg!(&test_mat);
    println!("Initial: {}", &test_mat);
    test_mat += test_mat.clone();
    println!("Double : {}", &test_mat);
    test_mat.sigmoid();
    println!("Sigmoid: {}", &test_mat);
}
