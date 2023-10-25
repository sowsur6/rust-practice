// #[derive(Debug,PartialEq, Eq)]
// pub struct Matrix(pub(i32, i32), pub(i32, i32));

// pub fn multiply(m: Matrix, multiplier: i32) -> Matrix {
//     let mut x = Matrix((0,0), (0,0));
//      x.0.0 = m.0.0*multiplier;
//       x.1.0 = m.1.0*multiplier;
//        x.0.1 = m.0.1*multiplier;
//         x.1.1 = m.1.1*multiplier;
//    return x;
// }



// fn main() {
//     let matrix = Matrix((1, 3), (4, 5));
//     println!("Original matrix {:?}", matrix);
//     println!("Matrix after multiply {:?}", multiply(matrix, 3));
// }
pub use m_multiplication::*;
pub mod m_multiplication {
    pub use crate::Matrix;

    pub fn multiply(matrix: Matrix, multiplier: i32) -> Matrix {
        matrix.multiply(multiplier)
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct Matrix(pub (i32, i32), pub (i32, i32));

impl Matrix {
    pub fn multiply(&self, multiplier: i32) -> Matrix {
        let (a, b) = self.0;
        let (c, d) = self.1;
        Matrix((a * multiplier, b * multiplier), (c * multiplier, d * multiplier))
    }
}

// use matrix_multiplication::*;

fn main() {
    let matrix = Matrix((1, 3), (4, 5));
    println!("Original matrix {:?}", matrix);
    println!("Matrix after multiply {:?}", multiply(matrix, 3));
}
mod tests {
    use super::*;
    use lib::{Kind, TestProperties};

    #[test]
    fn get_row() {
        let matrix: Matrix<u32> = Matrix(vec![vec![3, 6], vec![8, 0]]);
        let test = TestProperties {
            name: "row",
            kind: Kind::Method,
        };
        test.assert_with_message(&[Box::new(matrix.clone())], vec![3u32, 6], matrix.row(0));
        test.assert_with_message(&[Box::new(matrix.clone())], vec![8u32, 0], matrix.row(1));
    }

    #[test]
    fn get_col() {
        let matrix: Matrix<u32> = Matrix(vec![vec![3, 6], vec![8, 0]]);
        let test = TestProperties {
            name: "col",
            kind: Kind::Method,
        };
        test.assert_with_message(&[Box::new(matrix.clone())], matrix.col(0), vec![3u32, 8]);
        test.assert_with_message(&[Box::new(matrix.clone())], vec![6u32, 0], matrix.col(1));
    }

    #[test]
    fn matrix_multiplication() {
        let matrix_1: Matrix<u32> = Matrix(vec![vec![0, 1], vec![0, 0]]);
        let matrix_2: Matrix<u32> = Matrix(vec![vec![0, 0], vec![1, 0]]);
        let expected: Matrix<u32> = Matrix(vec![vec![1, 0], vec![0, 0]]);
        let test = TestProperties {
            name: "*",
            kind: Kind::Operator,
        };
        test.assert_with_message(
            &[Box::new(matrix_1.clone()), Box::new(matrix_2.clone())],
            matrix_1 * matrix_2,
            Some(expected),
        );

        let matrix_1: Matrix<u32> = Matrix(vec![vec![0, 1], vec![0, 0]]);
        let matrix_2: Matrix<u32> = Matrix(vec![vec![0, 0, 0], vec![1, 0, 0], vec![1, 1, 1]]);
        test.assert_with_message(
            &[Box::new(matrix_1.clone()), Box::new(matrix_2.clone())],
            matrix_1 * matrix_2,
            None,
        );
    }
}