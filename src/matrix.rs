use rand::Rng;

#[derive(Clone)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    pub data: Vec<f32>,
}

impl Matrix {
    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut f32 {
        assert!(self.data.len() > self.cols * row + col);
        &mut self.data[self.cols * row + col]
    }

    pub fn get(&self, row: usize, col: usize) -> &f32 {
        assert!(self.data.len() > self.cols * row + col);
        &self.data[self.cols * row + col]
    }

    pub fn alloc(rows: usize, cols: usize) -> Self {
        Matrix {
            rows,
            cols,
            data: vec![0.0; rows * cols], 
        }
    }

    pub fn fill(&mut self, value: f32) {
        self.data.clear();
        for _ in 0..self.data.capacity() {
            self.data.push(value);
        }
    }

    pub fn rand(&mut self) {
        let mut rng = rand::thread_rng();
        for _ in 0..self.data.capacity() {
            self.data.push(rng.gen());
        }
    }
}
// Multiplication
impl std::ops::Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        assert!(self.cols == rhs.rows);
        let mut dst: Matrix = Matrix::alloc(self.rows, rhs.cols);
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

impl std::ops::MulAssign for Matrix {
    fn mul_assign(&mut self, rhs: Self) {
        assert!(self.cols == rhs.rows);
        let mut dst: Matrix = Matrix::alloc(self.rows, rhs.cols);
        dst.fill(Default::default());
        for i in 0..dst.rows {
            for j in 0..dst.cols {
                for k in 0..self.cols {
                    *dst.get_mut(i, j) += *self.get(i, k) * *rhs.get(k, j);
                }
            }
        }
        *self = dst.clone();
    }
}

// Addition
impl std::ops::Add for Matrix {
    type Output = Matrix;

    fn add(mut self, rhs: Self) -> Self::Output {
        assert!(rhs.rows == self.rows);
        assert!(rhs.cols == self.cols);

        for i in 0..self.data.capacity() {
            self.data[i] = self.data[i] + rhs.data[i];
        }
        self
    }
}

impl std::ops::AddAssign for Matrix {
    fn add_assign(&mut self, rhs: Self) {
        assert!(rhs.rows == self.rows);
        assert!(rhs.cols == self.cols);

        for i in 0..self.data.capacity() {
            self.data[i] = self.data[i] + rhs.data[i];
        }
    }
}

// Debug
impl std::fmt::Debug for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "")?;
        write!(f, "[ ")?;
        for (i, x) in self.data.iter().enumerate() {
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

pub trait Sigmoid {
    fn sigmoid(&mut self);
}

impl Sigmoid for Matrix {
    fn sigmoid(&mut self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                let e = 2.71828f32;
                *self.get_mut(i, j) = 1.0f32 / (1.0f32 + (e.powf(*self.get(i, j))));
            }
        }
    }
}
