#[derive(Debug, Clone)]

pub struct Matrix(pub Vec<Vec<i32>>);

impl Matrix {
    pub fn new(slice: &[&[i32]]) -> Self{
        let matrix: Vec<Vec<i32>> = slice.iter()
            .map(|row| row.iter().copied().collect())
            .collect();
        Matrix(matrix)
    }
}

use std::fmt;

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut index = 0;
        for row in &self.0 {
            let row_str: Vec<String> = row.iter().map(|num| num.to_string()).collect();
            let mut row_formatted = row_str.join(" ");
            if index == &self.0.len()-1{
                row_formatted.insert(row_formatted.len() , ')');
                row_formatted.insert(0, '(');
            }else{
                row_formatted.insert(0, '(');
                row_formatted.push_str(")\n");
            }
            index +=1;
            write!(f, "{}", row_formatted)?;
        }
        Ok(())
    }
    
}
fn main() {
	let matrix = Matrix::new(&[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]]);
	println!("{}", matrix);
}

//output is (1 2 3)
//          (4 5 6)
//          (7 8 9)

#[cfg(test)]
mod tests {
    use super::*;
    // use lib::{Kind, TestProperties};
    // use matrix_display::*;

    #[test]
    fn it_works() {
        let matrix = Matrix::new(&[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]]);
        let display = String::from("(1 2 3)\n(4 5 6)\n(7 8 9)");
        let test = TestProperties {
            kind: Kind::Method,
            name: "to_string",
        };
        test.assert_with_message(&[Box::new(matrix.clone())], matrix.to_string(), display);
    }

    #[test]
    fn test_matrix_col() {
        let matrix = Matrix::new(&[&[1], &[2], &[3]]);
        let display = String::from("(1)\n(2)\n(3)");
        let test = TestProperties {
            kind: Kind::Method,
            name: "to_string",
        };
        test.assert_with_message(&[Box::new(matrix.clone())], matrix.to_string(), display);
    }

    #[test]
    fn test_matrix_row() {
        let matrix = Matrix::new(&[&[1, 2, 3]]);
        let display = String::from("(1 2 3)");
        let test = TestProperties {
            kind: Kind::Method,
            name: "to_string",
        };
        test.assert_with_message(&[Box::new(matrix.clone())], matrix.to_string(), display);
    }

    #[test]
    fn test_m_by_n_matrix() {
        let matrix = Matrix::new(&[&[1, 2, 3, 4, 5], &[6, 7, 8, 9, 10], &[11, 12, 13, 14, 15]]);
        let display = String::from("(1 2 3 4 5)\n(6 7 8 9 10)\n(11 12 13 14 15)");
        let test = TestProperties {
            kind: Kind::Method,
            name: "to_string",
        };
        test.assert_with_message(&[Box::new(matrix.clone())], matrix.to_string(), display);
    }

    #[test]
    fn test_empty_matrix() {
        let matrix = Matrix::new(&[&[]]);
        let display = String::from("()");
        let test = TestProperties {
            kind: Kind::Method,
            name: "to_string",
        };
        test.assert_with_message(&[Box::new(matrix.clone())], matrix.to_string(), display);
    }
}