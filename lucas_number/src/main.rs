pub fn lucas_number(n: u32) -> u32 {
    if n == 0 {
        return 2; // The first Lucas Number is 2
    }
    if n == 1 {
        return 1; // The second Lucas Number is 1
    }

    let mut prev = 2; // Initialize with the first Lucas Number
    let mut curr = 1; // Initialize with the second Lucas Number

    // Loop from 2 to n to calculate the nth Lucas Number
    for _ in 2..=n {
        let next = prev + curr; // Calculate the next Lucas Number
        prev = curr; // Update the previous number to be the current number
        curr = next; // Update the current number to be the next number
    }

    curr // Return the nth Lucas Number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lucas_number_test() {
        assert_eq!(lucas_number(2), 3);
        assert_eq!(lucas_number(5), 11);
        assert_eq!(lucas_number(10), 123);
        assert_eq!(lucas_number(13), 521);
        assert_eq!(lucas_number(25), 167761);
    }
}




fn main() {
    println!("The element in the position {} in Lucas Numbres is {}", 2, lucas_number(2));
    println!("The element in the position {} in Lucas Numbres is {}", 5, lucas_number(5));
    println!("The element in the position {} in Lucas Numbres is {}", 10, lucas_number(10));
    println!("The element in the position {} in Lucas Numbres is {}", 13, lucas_number(13));
}