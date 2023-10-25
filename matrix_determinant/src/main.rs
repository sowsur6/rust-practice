pub fn matrix_determinant(matrix: [[isize; 3]; 3]) -> isize {
    let a = matrix[0][0];
    let b = matrix[0][1];
    let c = matrix[0][2];

    let d = matrix[1][0];
    let e = matrix[1][1];
    let f = matrix[1][2];

    let g = matrix[2][0];
    let h = matrix[2][1];
    let i = matrix[2][2];

    let determinant = a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g);

    determinant
}

fn main() {
    let matrix = [[1, 2, 4], [2, -1, 3], [4, 0, 1]];

    println!(
        "The determinant of the matrix:\n|1  2  4|\n|2 -1  3|  = {}\n|4  0  1|",
        matrix_determinant(matrix)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_values() {
        assert_eq!(0, matrix_determinant([[1, 2, 3], [4, 5, 6], [7, 8, 9]]));
        assert_eq!(15, matrix_determinant([[6, 5, 4], [4, 6, 9], [1, 1, 2]]));
        assert_eq!(-1, matrix_determinant([[3, 0, 1], [4, 1, 2], [3, 2, 2]]));
    }

    #[test]
    fn also_negatvies() {
        assert_eq!(29, matrix_determinant([[1, -2, 2], [-1, 3, 6], [-3, 4, 7]]));
        assert_eq!(
            48,
            matrix_determinant([[-6, -1, 5], [-3, 2, 2], [1, 5, -5]])
        );
    }

    #[test]
    fn big_numbers() {
        assert_eq!(
            -1808088,
            matrix_determinant([[167, 181, 150], [164, 85, 111], [52, 91, 177]])
        );
    }
}

// output:
// The determinant of the matrix:
//|1  2  4|
//|2 -1  3|  = 35
//|4  0  1|
