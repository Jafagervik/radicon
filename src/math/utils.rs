use super::matrix::Matrix;
use std::fmt::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Reads content from a file and tries to make a matrix from it
/// TODO: Implement
pub fn read_matrix_from_file(path_str: &str) -> Result<Matrix, Error> {
    let path = Path::new(path_str);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(e) => panic!("Could not open file {}: {}", display, e),
        Ok(file) => file,
    };

    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    let mut matrix: [f32; 10] = [
        1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32,
    ];

    let rows = 10;
    let cols = 10;

    return Matrix::new(rows, cols, Vec::new());
}
