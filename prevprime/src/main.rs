pub fn prev_prime(nbr: u32) -> u32 {
    if nbr < 2 {
        return 0;
    }
    let mut i = nbr - 1;
    while i >= 2 {
        let mut is_prime = true;
        for j in 2..=i / 2 {
            if i % j == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            return i;
        }
        i -= 1;
    }
    prev_prime(i)
}


fn main() {
    println!("the previous prime number before 2 is: {}", prev_prime(2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prev_prime_test() {
        assert_eq!(0, prev_prime(0));
        assert_eq!(0, prev_prime(2));
        assert_eq!(2, prev_prime(3));
        assert_eq!(3, prev_prime(5));
        assert_eq!(31, prev_prime(34));
        assert_eq!(631, prev_prime(633));
        assert_eq!(478139, prev_prime(478152));
    }
}