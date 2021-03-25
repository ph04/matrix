use std::{error, fmt, ops};

pub const I_2: Matrix<2, 2> = Matrix {
    body: [
        [1.0, 0.0],
        [0.0, 1.0],
    ]
};

pub const I_3: Matrix<3, 3> = Matrix {
    body: [
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
    ]
};

pub const I_4: Matrix<4, 4> = Matrix {
    body: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
};

pub const R90_2: Matrix<2, 2> = Matrix {
    body: [
        [0.0, -1.0],
        [1.0,  0.0],
    ]
};

pub const R180_2: Matrix<2, 2> = Matrix {
    body: [
        [-1.0,  0.0],
        [ 0.0, -1.0],
    ]
};

pub const R270_2: Matrix<2, 2> = Matrix {
    body: [
        [ 0.0, 1.0],
        [-1.0, 0.0]
    ]
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix<const M: usize, const N: usize> {
    body: [[f32; N]; M],
}

impl<const M: usize, const N: usize> Matrix<M, N> {
    pub fn new(body: [[f32; N]; M]) -> Result<Self, MatrixError> {
        match (M, N) {
            (0, _) | (_, 0) => Err(MatrixError::EmptyDimension),
            (_, _) => Ok(Self {
                body
            })
        }
    }

    pub fn with_capacity() -> Self {
        Matrix {
            body: [[0.0; N]; M]
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (M, N)
    }

    pub fn get(&self, pos: (usize, usize)) -> f32 {
        self.body[pos.0][pos.1]
    }

    pub fn transpose(&self) -> Matrix<N, M> {
        let mut body = [[0.0; M]; N];

        for c in 0..N {
            for r in 0..M {
                body[c][r] = self.get((r, c))
            }
        }

        Matrix { body }
    }

    pub fn for_each_mut<F>(&mut self, function: F)
    where
        F: Fn(f32) -> f32
    {
        self.body.iter_mut().for_each(|row| row.iter_mut().for_each(|e| *e = function(*e)));
    }
}

impl<const M: usize, const N: usize> fmt::Display for Matrix<M, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.body.iter().try_for_each(|row| writeln!(f, "{:?}", row))
    }
}

impl<const M: usize, const N: usize> ops::Add for Matrix<M, N> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut body = [[0.0; N]; M];
        
        body.iter_mut().zip(self.body.iter().zip(&other.body)).for_each(|(rr, (rs, ro))| {
            rr.iter_mut().zip(rs.iter().zip(ro)).for_each(|(r, (s, o))| *r = s + o);
        });

        Self { body }
    }
}

impl<const M: usize, const N: usize> ops::Sub for Matrix<M, N> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut body = [[0.0; N]; M];
        
        body.iter_mut().zip(self.body.iter().zip(&other.body)).for_each(|(rr, (rs, ro))| {
            rr.iter_mut().zip(rs.iter().zip(ro)).for_each(|(r, (s, o))| *r = s - o);
        });

        Self { body }
    }
}

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

impl<const M: usize, const N: usize> ops::AddAssign for Matrix<M, N> {
    fn add_assign(&mut self, other: Self) {
        self.body.iter_mut().zip(&other.body).for_each(|(rs, ro)| {
            rs.iter_mut().zip(ro).for_each(|(s, o)| *s += o)
        });
    }
}

impl<const M: usize, const N: usize> ops::SubAssign for Matrix<M, N> {
    fn sub_assign(&mut self, other: Self) {
        self.body.iter_mut().zip(&other.body).for_each(|(rs, ro)| {
            rs.iter_mut().zip(ro).for_each(|(s, o)| *s -= o)
        });
    }
}

impl<const M: usize, const N: usize> ops::Add<f32> for Matrix<M, N> {
    type Output = Self;

    fn add(self, other: f32) -> Self {
        let mut body = [[0.0; N]; M];

        body.iter_mut().zip(&self.body).for_each(|(rr, rs)| {
            rr.iter_mut().zip(rs).for_each(|(b, s)| *b = s + other)
        });
        
        Self { body }
    }
}

impl<const M: usize, const N: usize> ops::Sub<f32> for Matrix<M, N> {
    type Output = Self;

    fn sub(self, other: f32) -> Self {
        let mut body = [[0.0; N]; M];

        body.iter_mut().zip(&self.body).for_each(|(rr, rs)| {
            rr.iter_mut().zip(rs).for_each(|(b, s)| *b = s - other)
        });

        Self { body }
    }
}

impl<const M: usize, const N: usize> ops::Mul<f32> for Matrix<M, N> {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        let mut body = [[0.0; N]; M];

        body.iter_mut().zip(&self.body).for_each(|(rr, rs)| {
            rr.iter_mut().zip(rs).for_each(|(b, s)| *b = s * other)
        });
        
        Self { body }
    }
}

impl<const M: usize, const N: usize> ops::Div<f32> for Matrix<M, N> {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        let mut body = [[0.0; N]; M];

        body.iter_mut().zip(&self.body).for_each(|(rr, rs)| {
            rr.iter_mut().zip(rs).for_each(|(b, s)| *b = s / other)
        });
        
        Self { body }
    }
}

impl<const M: usize, const N: usize> ops::AddAssign<f32> for Matrix<M, N> {
    fn add_assign(&mut self, other: f32) {
        self.body.iter_mut().for_each(|row| row.iter_mut().for_each(|e| *e += other));
    }
}

impl<const M: usize, const N: usize> ops::SubAssign<f32> for Matrix<M, N> {
    fn sub_assign(&mut self, other: f32) {
        self.body.iter_mut().for_each(|row| row.iter_mut().for_each(|e| *e -= other));
    }
}

impl<const M: usize, const N: usize> ops::MulAssign<f32> for Matrix<M, N> {
    fn mul_assign(&mut self, other: f32) {
        self.body.iter_mut().for_each(|row| row.iter_mut().for_each(|e| *e *= other));
    }
}

impl<const M: usize, const N: usize> ops::DivAssign<f32> for Matrix<M, N> {
    fn div_assign(&mut self, other: f32) {
        self.body.iter_mut().for_each(|row| row.iter_mut().for_each(|e| *e /= other));
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MatrixError {
    EmptyDimension,
}

impl fmt::Display for MatrixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MatrixError::EmptyDimension => write!(f, "The dimensions of the matrix can't be 0."),
        }
    }
}

impl error::Error for MatrixError {}
