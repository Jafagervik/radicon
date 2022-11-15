use rand::Rng;
use std::vec;

trait MatrixOps<T> {
    fn add(&self, other: &Matrix<T>) -> Matrix<T>;
    fn sub(&self, other: &Matrix<T>) -> Matrix<T>;
    fn mult(&self, other: &Matrix<T>) -> Matrix<T>;
    fn div(&self, other: &Matrix<T>) -> Matrix<T>;

    fn crossmult(&self, other: &Matrix<T>) -> Matrix<T>;
    fn eigenvector(&self, other: &Matrix<T>) -> Matrix<T>;
    fn projection(&self, other: &Matrix<T>) -> Matrix<T>;
    fn determinant(&self) -> T;

    // Bys
    fn add_by(&mut self, number: T);
    fn sub_by(&mut self, number: T);
    fn mult_by(&mut self, number: T);
    fn div_by(&mut self, number: T);
}

#[derive(Default, Debug, Clone)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T> Matrix<T> {
    pub fn same_dims(&self, other: &Matrix<T>) -> bool {
        return self.rows == other.rows && self.cols == other.cols;
    }

    pub fn new(rows: usize, cols: usize, data: Vec<T>) -> Self {
        Matrix { rows, cols, data }
    }

    pub fn at(&self, row: usize, col: usize) -> T {
        let idx = self.cols * row + col;

        return self.data[idx];
    }

    pub fn get(&self, idx: usize) -> T {
        self.data[idx]
    }

    pub fn idx(&self, row: usize, col: usize) -> usize {
        self.cols * row + col
    }

    pub fn zeros(&self, rows: usize, cols: usize) -> Self {
        return Matrix::new(rows, cols, vec![0_f32; rows * cols]);
    }

    pub fn ones(&self, rows: usize, cols: usize) -> Self {
        return Matrix::new(rows, cols, vec![1_f32; rows * cols]);
    }

    pub fn ns(&self, rows: usize, cols: usize, number: T) -> Self {
        return Matrix::new(rows, cols, vec![number; rows * cols]);
    }

    pub fn fill(&mut self, number: T) {
        self.data = vec![number; self.rows * self.cols]
    }

    pub fn rand(rows: usize, cols: usize, min: Option<T>, max: Option<T>) -> Self {
        let mut rng = rand::thread_rng();
        let mut data = vec![0_f32; (rows * cols) as usize];

        let min = min.unwrap_or(0_f32);
        let max = max.unwrap_or(1000_f32);

        for i in 0..(rows * cols) {
            let number: T = rng.gen_range(min..=max);

            data[i] = number;
        }

        return Matrix::new(rows, cols, data);
    }

    pub fn transpose(&self) -> Matrix<T> {
        let mut data = vec![0_f32; self.cols * self.rows];

        for r in 0..self.rows as usize {
            for c in 0..self.cols as usize {
                let idx_new = c * self.rows + r;

                data[idx_new] = self.at(r, c);
            }
        }

        return Matrix::new(self.cols, self.rows, data);
    }

    pub fn t(&self) -> Matrix<T> {
        self.transpose()
    }
}

impl<T> MatrixOps<T> for Matrix<T> {
    fn add(&self, other: &Matrix<T>) -> Matrix<T> {
        if !self.same_dims(other) {
            panic!("The two matrices do not share dimensions!");
        }

        let data_r: Vec<T> = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a + b)
            .collect();

        return Matrix::new(self.rows, self.cols, data_r);
    }

    fn sub(&self, other: &Matrix<T>) -> Matrix<T> {
        if !self.same_dims(other) {
            panic!("The two matrices do not share dimensions!");
        }

        let data_r: Vec<f32> = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a - b)
            .collect();

        return Matrix::new(self.rows, self.cols, data_r);
    }

    /// TODO: Multithread
    /// TODO: Cached implementation
    fn mult(&self, other: &Matrix<T>) -> Matrix<T> {
        if self.cols != other.rows {
            panic!("The two matrices can not be multiplied!");
        }

        let rows = self.rows;
        let cols = other.cols;

        let mut result = Matrix::new(rows, cols, vec![0_f32; rows * cols]);

        for r in 0..rows {
            for c in 0..cols {
                for k in 0..other.rows as usize {
                    result.data[result.idx(r, c)] +=
                        self.data[self.idx(r, k)] * other.data[other.idx(k, c)];
                }
            }
        }

        return result;
    }

    // TODO: Catch the else
    fn div(&self, other: &Matrix<T>) -> Matrix<T> {
        if !self.same_dims(other) {
            panic!("The two matrices do not share dimensions!");
        }

        let data_r: Vec<T> = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a / b)
            .collect();

        return Matrix::new(self.rows, self.cols, data_r);
    }

    fn crossmult(&self, other: &Matrix<T>) -> Matrix<T> {
        return Matrix::default();
    }

    fn eigenvector(&self, other: &Matrix<T>) -> Matrix<T> {
        return Matrix::default();
    }

    fn projection(&self, other: &Matrix<T>) -> Matrix<T> {
        return Matrix::default();
    }

    /// Gets the determinant of a vector
    /// Checks the dimension and gets up to ND
    fn determinant(&self) -> T {
        return 0_f32;
    }

    // ================================================
    //                   Bys
    // ================================================
    fn add_by(&mut self, number: T) {
        self.data.into_iter().map(|a| a = a + number);
    }

    fn sub_by(&mut self, number: T) {
        self.data.into_iter().map(|a| a = a - number);
    }

    fn mult_by(&mut self, number: T) {
        self.data.into_iter().map(|a| a = a * number);
    }

    fn div_by(&mut self, number: T) {
        if number == 0_f32 {
            panic!("Lol, go learn some math kiddo");
        }

        self.data.into_iter().map(|a| a = a / number);
    }
}
