#![allow(unused_variables)]

use matrix::matrix::Matrix;

fn main() {
    let m1 = Matrix::new([
        [1.0, 4.0, 6.0],
    ]).unwrap();

    let m2 = Matrix::new([
        [2.0, 3.0],
        [5.0, 8.0],
        [7.0, 9.0]
    ]).unwrap();

    let m3 = Matrix::new([
        [1.0, 2.0],
        [3.0, 4.0],
    ]).unwrap();

    println!("{}", m3 * m3);
}