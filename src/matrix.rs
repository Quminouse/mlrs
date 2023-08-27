#[derive(Clone, Debug)]
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

    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut T {
        assert!(self.data.len() > self.cols * row + col);
        &mut self.data[self.cols * row + col]
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        assert!(self.data.len() > self.cols * row + col);
        &self.data[self.cols * row + col]
    }

    pub fn new(rows: usize, cols: usize) -> Self
    where
        T: Default + Clone,
    {
        Matrix {
            rows,
            cols,
            data: vec![Default::default(); rows * cols],
        }
    }

    pub fn fill(&mut self, value: T)
    where
        T: Copy,
    {
        for i in self.iter_mut() {
            *i = value;
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
        let mut dst: Matrix<T> = Matrix::new(self.rows, rhs.cols);
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

// Addition
impl<T: std::ops::Add<Output = T> + Copy> std::ops::Add for Matrix<T> {
    type Output = Matrix<T>;

    fn add(mut self, rhs: Self) -> Self::Output {
        assert!(rhs.rows == self.rows);
        assert!(rhs.cols == self.cols);

        self.iter_mut()
            .zip(rhs.iter())
            .for_each(|(x, y)| *x = *x + *y);
        self
    }
}

// Debug
impl<T: std::fmt::Display> std::fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        if self.data.capacity() == 0 {
            return write!(f, "[]");
        }
        write!(f, "[ ")?;
        for (i, x) in self.iter().enumerate() {
            write!(f, "{:.2} ", x)?;
            if (i + 1) >= self.data.capacity() {
                write!(f, "]")?;
            } else if (i + 1) % self.cols == 0 {
                write!(f, "\n   ")?;
            }
        }
        Ok(())
    }
}
