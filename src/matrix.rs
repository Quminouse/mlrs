use rand::{Rng, prelude::Distribution, distributions::Standard};

pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T: Copy> Matrix<T> {
    pub fn alloc(rows: usize, cols: usize) -> Self {
        Matrix {
            rows,
            cols,
            data: Vec::with_capacity(rows * cols),
        }
    }
    pub fn fill(&mut self, value: T) {
        self.data.clear();
        for _ in 0..self.data.capacity() {
            self.data.push(value);
        }
    }
    fn get_mut(&mut self, row: usize, col: usize) -> &mut T {
        &mut self.data[self.cols * row + col]
    }
    fn get(&self, row: usize, col: usize) -> &T {
        &self.data[self.cols * row + col]
    }
}

impl <T: Copy> Matrix<T> where Standard: Distribution<T> {
    pub fn rand(&mut self) {
        self.data.clear();
        let mut rng = rand::thread_rng();
        for _ in 0..self.data.capacity() {
            self.data.push(rng.gen());
        }
    }
}

// Multiplication
impl<T: std::ops::Mul<Output = T> + Copy + std::default::Default + std::ops::AddAssign>
    std::ops::Mul for Matrix<T>
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        assert!(self.cols == rhs.rows);
        let mut dst: Matrix<T> = Matrix::alloc(self.rows, rhs.cols);
        dst.fill(Default::default());
        for i in 0..dst.rows {
            for j in 0..dst.cols {
                for k in 0..self.cols {
                    *dst.get_mut(i, j) += *self.get(i, k) * *rhs.get(k, j);
                }
            }
        }
        dst
    }
}

impl<T: std::ops::Mul<Output = T> + Copy + std::default::Default + std::ops::AddAssign>
    std::ops::MulAssign for Matrix<T>
{
    fn mul_assign(&mut self, rhs: Self) {
        assert!(self.cols == rhs.rows);
        let mut dst: Matrix<T> = Matrix::alloc(self.rows, rhs.cols);
        dst.fill(Default::default());
        for i in 0..dst.rows {
            for j in 0..dst.cols {
                for k in 0..self.cols {
                    *dst.get_mut(i, j) += *self.get(i, k) * *rhs.get(k, j);
                }
            }
        }
    }
}

// Addition
impl<T: std::ops::Add<Output = T> + Copy> std::ops::Add for Matrix<T> {
    type Output = Matrix<T>;

    fn add(mut self, rhs: Self) -> Self::Output {
        assert!(rhs.rows == self.rows);
        assert!(rhs.cols == self.cols);

        for i in 0..self.data.capacity() {
            self.data[i] = self.data[i] + rhs.data[i];
        }
        self
    }
}

impl<T: std::ops::Add<Output = T> + Copy> std::ops::AddAssign for Matrix<T> {
    fn add_assign(&mut self, rhs: Self) {
        assert!(rhs.rows == self.rows);
        assert!(rhs.cols == self.cols);

        for i in 0..self.data.capacity() {
            self.data[i] = self.data[i] + rhs.data[i];
        }
    }
}

// Debug
impl<T: std::fmt::Debug + Copy> std::fmt::Debug for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "")?;
        write!(f, "[ ")?;
        for (i, x) in self.data.iter().enumerate() {
            write!(f, "{:.2?} ", x)?;
            if (i + 1) == self.data.capacity() {
                write!(f, "]")?;
            } else if (i + 1) % self.cols == 0 {
                write!(f, "\n  ")?;
            }         }
        Ok(())
    }
}
