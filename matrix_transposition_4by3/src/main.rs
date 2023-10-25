#[derive(Debug, PartialEq, Eq)]
pub struct Matrix4by3(
    pub (i32, i32, i32),
    pub (i32, i32, i32),
    pub (i32, i32, i32),
    pub (i32, i32, i32),
);
#[derive(Debug, PartialEq, Eq)]
pub struct Matrix3by4(
    pub (i32, i32, i32, i32),
    pub (i32, i32, i32, i32),
    pub (i32, i32, i32, i32),
);

pub fn transpose(m: Matrix4by3) -> Matrix3by4 {
    let (a, b, c) = m.0;
    let (d, e, f) = m.1;
    let (g, h, i) = m.2;
    let (j, k, l) = m.3;
    

    Matrix3by4(
        (a, d, g, j),
        (b, e, h, k),
        (c, f, i, l),
        
    )
}


fn main() {
    let matrix = Matrix4by3((1, 2, 3), (4, 5, 6), (7, 8, 9), (10, 11, 12));
    println!("Original matrix {:?}", matrix);
    println!("Transpose matrix {:?}", transpose(matrix));
}


#[cfg(test)]
mod tests {
    // use matrix_transposition_4by3::*;
    use super::*;

    #[test]
    fn test_tranposion() {
        let matrix = Matrix4by3((1, 2, 3), (4, 5, 6), (7, 8, 9), (10, 11, 12));
        let expected = Matrix3by4((1, 4, 7, 10), (2, 5, 8, 11), (3, 6, 9, 12));
        assert_eq!(transpose(matrix), expected);
    }
}