//! This crate provides a `Matrix` structure, with many helpful
//! trait implementations to perform calculations between
//! matrices, but with absolutely no allocations.

use std::{fmt, ops};

/// The identity matrix `2x2`.
pub const I_2: Matrix<2, 2> = Matrix {
    body: [
        [1.0, 0.0],
        [0.0, 1.0],
    ]
};

/// The identity matrix `3x3`.
pub const I_3: Matrix<3, 3> = Matrix {
    body: [
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
    ]
};

/// The identity matrix `4x4`.
pub const I_4: Matrix<4, 4> = Matrix {
    body: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
};

/// The `90°` rotation matrix `2x2`.
pub const R90_2: Matrix<2, 2> = Matrix {
    body: [
        [0.0, -1.0],
        [1.0,  0.0],
    ]
};

/// The `180°` rotation matrix `2x2`.
pub const R180_2: Matrix<2, 2> = Matrix {
    body: [
        [-1.0,  0.0],
        [ 0.0, -1.0],
    ]
};

/// The `270°` rotation matrix `2x2`.
pub const R270_2: Matrix<2, 2> = Matrix {
    body: [
        [ 0.0, 1.0],
        [-1.0, 0.0]
    ]
};

/// A struct that represents a Matrix
/// with `M` rows and `N` columns.
/// 
/// # Examples
/// 
/// ```
/// # pub use small_matrix::matrix::Matrix;
/// let matrix = Matrix::new([
///     [2.0, 3.0],
///     [5.0, 8.0],
///     [7.0, 9.0]
/// ]);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix<const M: usize, const N: usize> {
    body: [[f32; N]; M],
}

impl<const M: usize, const N: usize> Matrix<M, N> {
    /// Returns a new matrix based on
    /// the given array of [[f32; N]; M].
    /// 
    /// # Examples
    /// 
    /// ```
    /// # pub use small_matrix::matrix::Matrix;
    /// let matrix = Matrix::new([
    ///     [-1.1, 4.2],
    ///     [2.4, 3.6],
    /// ]);
    /// ```
    pub fn new(body: [[f32; N]; M]) -> Self {
        Self { body }
    }

    /// Returns a matrix with the given
    /// dimensions with `0.0`s.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # pub use small_matrix::matrix::Matrix;
    /// let zeros = Matrix::<5, 4>::zeros(); // fills the matrix with zeros
    /// ```
    pub fn zeros() -> Self {
        Self {
            body: [[0.0; N]; M]
        }
    }

    /// Returns a matrix with the given
    /// dimensions with `n`s.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # pub use small_matrix::matrix::Matrix;
    /// let threes = Matrix::<3, 2>::fill(3.0); // fills the matrix with threes
    /// ```
    pub fn fill(n: f32) -> Self {
        Self {
            body: [[n; N]; M]
        }
    }

    /// Returns the size of the matrix, `(M, N)`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # pub use small_matrix::matrix::Matrix;
    /// let matrix = Matrix::<3, 5>::zeros(); // fills the matrix with zeros
    /// 
    /// assert_eq!(matrix.size(), (3, 5));
    /// ```
    pub fn size(&self) -> (usize, usize) {
        (M, N)
    }

    /// Returns an `Option<f32>`, with the element placed on the
    /// `pos.1`-nth column, on the `pos.0`-nth row, if
    /// `pos.0` is less than `M` and `pos.1` is less than `N`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # pub use small_matrix::matrix::Matrix;
    /// let matrix = Matrix::new([
    ///     [1.0, 2.0],
    ///     [3.0, 4.0],
    /// ]);
    /// 
    /// assert_eq!(matrix.get((0, 1)).unwrap(), 2.0); // first row, second column
    /// ```
    pub fn get(&self, pos: (usize, usize)) -> Option<f32> {
        if pos.0 < M && pos.1 < N {
            Some(self.body[pos.0][pos.1])
        } else {
            None
        }
    }

    /// Returns the given matrix transposed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # pub use small_matrix::matrix::Matrix;
    /// // this is a `2x3` matrix
    /// let matrix = Matrix::new([
    ///     [1.2, 3.4, 7.3],
    ///     [3.6, 9.4, 0.6],
    /// ]);
    /// 
    /// // this is a `3x2` matrix
    /// let transposed = matrix.transpose();
    /// 
    /// assert_eq!(transposed.size(), (3, 2));
    /// ```
    pub fn transpose(&self) -> Matrix<N, M> {
        let mut body = [[0.0; M]; N];

        body.iter_mut().enumerate().for_each(|(c, row)| {
            row.iter_mut().enumerate().for_each(|(r, e)| *e = self.get((r, c)).unwrap())
        });

        Matrix { body }
    }

    /// Swaps the rows with the corresponding given indexes.
    /// 
    /// # Panics
    /// 
    /// Panics if `idx_1` or `idx_2` are out of bounds.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # pub use small_matrix::matrix::Matrix;
    /// let mut matrix = Matrix::new([
    ///     [1.0, 3.0],
    ///     [2.0, 4.0],
    /// ]);
    /// 
    /// matrix.swap_rows(0, 1); // swaps the first and the second row
    /// 
    /// assert_eq!(matrix.get((0, 0)).unwrap(), 2.0); // now the two rows are swapped
    /// ```
    pub fn swap_rows(&mut self, idx_1: usize, idx_2: usize) {
        self.body.swap(idx_1, idx_2);
    }

    /// Applies the given function to every
    /// element of the matrix.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # pub use small_matrix::matrix::Matrix;
    /// let mut fives = Matrix::<2, 4>::fill(5.0); // fills the matrix with fives
    /// 
    /// fives.for_each(|element| *element += 2.0);
    /// 
    /// assert_eq!(fives.get((0, 0)).unwrap(), 7.0); // every element is now `7.0`
    /// ```
    pub fn for_each<F: FnMut(&mut f32)>(&mut self, mut function: F) {
        self.body.iter_mut().for_each(|row| row.iter_mut().for_each(|e| function(e)));
    }
}

impl<const M: usize, const N: usize> fmt::Display for Matrix<M, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.body.iter().try_for_each(|row| writeln!(f, "{:?}", row))
    }
}

/// A macro used to implement `Add` and `Sub`.
macro_rules! impl_ops {
    ($trait:ident, $func:ident, $op:tt) => {
        impl<const M: usize, const N: usize> ops::$trait for Matrix<M, N> {
            type Output = Self;

            fn $func(self, other: Self) -> Self {
                let mut body = [[0.0; N]; M];
                
                body.iter_mut().zip(self.body.iter().zip(&other.body)).for_each(|(rr, (rs, ro))| {
                    rr.iter_mut().zip(rs.iter().zip(ro)).for_each(|(r, (s, o))| *r = s $op o);
                });

                Self { body }
            }
        }
    };
}

impl_ops!(Add, add, +);
impl_ops!(Sub, sub, -);

impl<const M: usize, const L: usize, const N: usize> ops::Mul<Matrix<L, N>> for Matrix<M, L> {
    type Output = Matrix<M, N>;
    
    fn mul(self, other: Matrix<L, N>) -> Matrix<M, N> {
        let mut body = [[0.0; N]; M];

        let other_t = other.transpose();

        body.iter_mut().zip(&self.body).for_each(|(rr, rs)| {
            rr.iter_mut().zip(&other_t.body).for_each(|(r, ro)| {
                *r = rs.iter().zip(ro).fold(0.0, |acc, (s, o)| acc + s * o);
            });
        });

        Matrix { body }
    }
}

/// A macro used to implement
/// `AddAssign` and `SubAssign`.
macro_rules! impl_ops_assign {
    ($trait_assign:ident, $func_assign:ident, $op_assign:tt) => {
        impl<const M: usize, const N: usize> ops::$trait_assign for Matrix<M, N> {
            fn $func_assign(&mut self, other: Self) {
                self.body.iter_mut().zip(&other.body).for_each(|(rs, ro)| {
                    rs.iter_mut().zip(ro).for_each(|(s, o)| *s $op_assign o)
                });
            }
        }
    };
}

impl_ops_assign!(AddAssign, add_assign, +=);
impl_ops_assign!(SubAssign, sub_assign, -=);

/// A macro used to implement
/// `Add<f32>`, `Sub<f32>`,
/// `Mul<f32>` and `Div<f32>`.
macro_rules! impl_opsf32 {
    ($trait:ident, $func:ident, $op:tt) => {
        impl<const M: usize, const N: usize> ops::$trait<f32> for Matrix<M, N> {
            type Output = Self;

            fn $func(self, other: f32) -> Self {
                let mut body = [[0.0; N]; M];

                body.iter_mut().zip(&self.body).for_each(|(rr, rs)| {
                    rr.iter_mut().zip(rs).for_each(|(b, s)| *b = s $op other)
                });
                
                Self { body }
            }
        }
    };
}

impl_opsf32!(Add, add, +);
impl_opsf32!(Sub, sub, -);
impl_opsf32!(Mul, mul, *);
impl_opsf32!(Div, div, /);

/// A macro used to implement
/// `AddAssign<f32>`, `SubAssign<f32>`,
/// `MulAssign<f32>` and `DivAssign<f32>`.
macro_rules! impl_ops_assignf32 {
    ($trait_assign:ident, $func_assign:ident, $op_assign:tt) => {
        impl<const M: usize, const N: usize> ops::$trait_assign<f32> for Matrix<M, N> {
            fn $func_assign(&mut self, other: f32) {
                self.body.iter_mut().for_each(|row| row.iter_mut().for_each(|e| *e $op_assign other));
            }
        }
    };
}

impl_ops_assignf32!(AddAssign, add_assign, +=);
impl_ops_assignf32!(SubAssign, sub_assign, -=);
impl_ops_assignf32!(MulAssign, mul_assign, *=);
impl_ops_assignf32!(DivAssign, div_assign, /=);