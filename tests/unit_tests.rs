#[cfg(test)]
mod tests {
    use small_matrix::matrix::*;

    #[test]
    fn i_2_test() {
        let matrix = Matrix::new([
            [1.0, 2.0],
            [3.0, 4.0],
        ]);

        assert_eq!(matrix * I_2, I_2 * matrix);
        assert_eq!(matrix * I_2, matrix);
    }

    #[test]
    fn i_3_test() {
        let matrix = Matrix::new([
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0],
        ]);

        assert_eq!(matrix * I_3, I_3 * matrix);
        assert_eq!(matrix * I_3, matrix);
    }

    #[test]
    fn i_4_test() {
        let matrix = Matrix::new([
            [ 1.0,  2.0,  3.0,  4.0],
            [ 5.0,  6.0,  7.0,  8.0],
            [ 9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);

        assert_eq!(matrix * I_4, I_4 * matrix);
        assert_eq!(matrix * I_4, matrix);
    }

    #[test]
    fn r90_2_test() {        
        let matrix = Matrix::new([
            [1.0],
            [2.0],
        ]);

        let rotated = Matrix::new([
            [-2.0],
            [ 1.0],
        ]);

        assert_eq!(R90_2 * matrix, rotated);
    }

    #[test]
    fn r180_2_test() {        
        let matrix = Matrix::new([
            [1.0],
            [2.0],
        ]);

        let rotated = Matrix::new([
            [-1.0],
            [-2.0],
        ]);

        assert_eq!(R180_2 * matrix, rotated);
    }

    #[test]
    fn r270_2_test() {        
        let matrix = Matrix::new([
            [1.0],
            [2.0],
        ]);

        let rotated = Matrix::new([
            [ 2.0],
            [-1.0],
        ]);

        assert_eq!(R270_2 * matrix, rotated);
    }

    #[test]
    fn e2_vector_matrix_multiplication() {
        let matrix = Matrix::new([
            [1.0, 2.0],
            [3.0, 4.0],
        ]);
        
        let e2_vector = Matrix::new([
            [0.0],
            [1.0]
        ]);

        let result = Matrix::new([
            [2.0],
            [4.0],
        ]);

        assert_eq!(matrix * e2_vector, result);
    }

    #[test]
    fn e3_vector_matrix_multiplication() {
        let matrix = Matrix::new([
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0],
        ]);
        
        let e3_vector = Matrix::new([
            [0.0],
            [0.0],
            [1.0],
        ]);

        let result = Matrix::new([
            [3.0],
            [6.0],
            [9.0],
        ]);

        assert_eq!(matrix * e3_vector, result);
    }
}