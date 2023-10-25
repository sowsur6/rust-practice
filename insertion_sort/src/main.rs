pub fn insertion_sort(slice: &mut [i32], steps: usize) {
    let len = slice.len();

    for i in 1..len {
        let mut j = i;

        while j > 0 && slice[j] < slice[j - 1] {
            slice.swap(j, j - 1);
            j -= 1;
        }

        if i >= steps {
            return;
        }
    }
}


fn main() {
    let mut target = [5, 3, 7, 2, 1, 6, 8, 4];
    // executes the first iteration of the algorithm
    insertion_sort(&mut target, 1);
    println!("{:?}", target);

    let mut target = [5, 3, 7, 2, 1, 6, 8, 4];
    let len = target.len();
    // executes len - 1 iterations of the algorithm
    // i.e. sorts the slice
    insertion_sort(&mut target, len - 1);
    println!("{:?}", target);
}

#[cfg(test)]
mod tests {
    // use crate insertion_sort::*;
  use super::*;
    #[test]
    fn it_works() {
        let mut target = [5, 3, 7, 2, 1, 6, 8, 4];
        let len = target.len();
        insertion_sort(&mut target, len - 1);
        assert_eq!(&[1, 2, 3, 4, 5, 6, 7, 8], &target);
    }

    #[test]
    fn test_first_step() {
        let mut target = [5, 3, 7, 2, 1, 6, 8, 4];
        insertion_sort(&mut target, 1);
        assert_eq!(&[3, 5, 7, 2, 1, 6, 8, 4], &target);
    }

    #[test]
    fn test_second_step() {
        let mut target = [5, 3, 7, 2, 1, 6, 8, 4];
        insertion_sort(&mut target, 2);
        assert_eq!(&[3, 5, 7, 2, 1, 6, 8, 4], &target);
    }
}