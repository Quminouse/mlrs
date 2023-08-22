use rand::{distributions::Standard, prelude::Distribution, Rng};

#[derive(Clone)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T> Matrix<T> {
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.data.iter_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.data.iter()
    }
}

impl<T: Clone + Copy> Matrix<T> {
    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut T {
        assert!(self.data.len() > self.cols * row + col);
        &mut self.data[self.cols * row + col]
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        assert!(self.data.len() > self.cols * row + col);
        &self.data[self.cols * row + col]
    }

    pub fn alloc(rows: usize, cols: usize) -> Self {
        Matrix {
            rows,
            cols,
            data: Vec::with_capacity(rows * cols),
        }
    }

    pub fn fill(&mut self, value: &T) {
        for i in self.iter_mut() {
            *i = *value;
        }
    }

    pub fn rand(&mut self)
    where
        Standard: Distribution<T>,
    {
        let mut rng = rand::thread_rng();
        for i in self.iter_mut() {
            *i = rng.gen();
        }
    }
}
// Multiplication
impl<T: std::ops::Add<Output = T> + std::ops::Mul<Output = T> + Default + Clone + Copy>
    std::ops::Mul for Matrix<T>
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        assert!(self.cols == rhs.rows);
        let mut dst: Matrix<T> = Matrix::alloc(self.rows, rhs.cols);
        for i in 0..dst.rows {
            for j in 0..dst.cols {
                for k in 0..self.cols {
                    *dst.get_mut(i, j) = T::default() + *self.get(i, k) * *rhs.get(k, j);
                }
            }
        }
        dst
    }
}

impl<T: std::ops::Add<Output = T> + std::ops::Mul<Output = T> + Default + Clone + Copy>
    std::ops::MulAssign for Matrix<T>
{
    fn mul_assign(&mut self, rhs: Self) {
        assert!(self.cols == rhs.rows);
        let mut dst: Matrix<T> = Matrix::alloc(self.rows, rhs.cols);
        for i in 0..dst.rows {
            for j in 0..dst.cols {
                for k in 0..self.cols {
                    *dst.get_mut(i, j) = T::default() + *self.get(i, k) * *rhs.get(k, j);
                }
            }
        }
        *self = dst.clone();
    }
}

// Addition
impl<T: std::ops::Add<Output = T> + Copy> std::ops::Add for Matrix<T> {
    type Output = Matrix<T>;

    fn add(mut self, rhs: Self) -> Self::Output {
        assert!(rhs.rows == self.rows);
        assert!(rhs.cols == self.cols);

        let _ = self.iter_mut().zip(rhs.iter()).map(|(x, y)| {
            *x = *x + *y;
        });
        self
    }
}

impl<T: std::ops::Add<Output = T> + Copy> std::ops::AddAssign for Matrix<T> {
    fn add_assign(&mut self, rhs: Self) {
        assert!(rhs.rows == self.rows);
        assert!(rhs.cols == self.cols);

        let _ = self.iter_mut().zip(rhs.iter()).map(|(x, y)| {
            *x = *x + *y;
        });
    }
}

// Debug
impl<T: std::fmt::Debug> std::fmt::Debug for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "")?;
        write!(f, "[ ")?;
        for (i, x) in self.iter().enumerate() {
            write!(f, "{:.2?} ", x)?;
            if (i + 1) == self.data.capacity() {
                write!(f, "]")?;
            } else if (i + 1) % self.cols == 0 {
                write!(f, "\n  ")?;
            }
        }
        Ok(())
    }
}
