mod matrix;
use std::vec;

use matrix::{Matrix, Sigmoid};

fn forward(
    a: &mut Vec<Matrix>,
    w: &mut Vec<Matrix>,
    b: &mut Vec<Matrix>,
    x1: &f32,
    x2: &f32,
) -> f32 {
    *a[0].get_mut(0, 0) = *x1;
    *a[0].get_mut(0, 1) = *x2;

    a[1] = (a[0].clone() * w[0].clone()) + b[0].clone();
    a[1].sigmoid();

    a[2] = (a[1].clone() * w[1].clone()) + b[1].clone();
    a[2].sigmoid();

    return *a[2].get(0, 0);
}

fn main() {
    let mut a0 = Matrix::alloc(1, 2);

    let mut w1 = Matrix::alloc(2, 2);
    let mut b1 = Matrix::alloc(1, 2);
    let mut a1 = Matrix::alloc(1, 2);

    let mut w2 = Matrix::alloc(2, 1);
    let mut b2 = Matrix::alloc(1, 1);
    let mut a2 = Matrix::alloc(1, 1);

    w1.rand();
    b1.rand();
    w2.rand();
    b2.rand();

    let mut a = vec![a0, a1, a2];
    let mut b = vec![b1, b2];
    let mut w = vec![w1, w2];

    let mut x1 = 0.0f32;
    let mut x2 = 1.0f32;

    println!("{}", forward(&mut a, &mut w, &mut b, &x1, &x2));
}
