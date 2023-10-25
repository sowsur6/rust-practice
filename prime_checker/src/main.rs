#[derive(PartialEq, Eq, Debug)]
pub enum PrimeErr {
    Even,
    Divider(u32),
}

pub fn prime_checker(nb: u32) -> Option<Result<u32, PrimeErr>> {
    if nb <= 1 {
        return None; 
    }
    if nb == 2 {
        return Some(Ok(nb));
    }
    if nb % 2 == 0 {
        return Some(Err(PrimeErr::Even)); 
    }

    let mut divider = 3;
    while divider * divider <= nb {
        if nb % divider == 0 {
            return Some(Err(PrimeErr::Divider(divider))); // Return Some(Err) if the number has a smaller divider
        }
        divider += 2; // Skip even numbers as potential dividers
    }

    Some(Ok(nb)) // Return Some(Ok) if the number is prime
}


fn main() {
    println!("Is {} prime? {:?}", 2, prime_checker(2));
    println!("Is {} prime? {:?}", 14, prime_checker(14));
    println!("Is {} prime? {:?}", 2147483647, prime_checker(2147483647));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_checker_prime() {
        assert_eq!(Some(Ok(3)), prime_checker(3));
        assert_eq!(Some(Ok(5)), prime_checker(5));
        assert_eq!(Some(Ok(7)), prime_checker(7));
        assert_eq!(Some(Ok(23)), prime_checker(23));
        assert_eq!(Some(Ok(7919)), prime_checker(7919));
    }
    #[test]
    fn prime_checker_even() {
        assert_eq!(Some(Err(PrimeErr::Even)), prime_checker(4));
        assert_eq!(Some(Err(PrimeErr::Even)), prime_checker(6));
        assert_eq!(Some(Err(PrimeErr::Even)), prime_checker(1234));
    }
    #[test]
    fn prime_checker_not_prime() {
        assert_eq!(Some(Err(PrimeErr::Divider(3))), prime_checker(9));
        assert_eq!(Some(Err(PrimeErr::Divider(5))), prime_checker(25));
        assert_eq!(Some(Err(PrimeErr::Divider(11))), prime_checker(121));
    }
    #[test]
    fn prime_checker_small() {
        assert_eq!(None, prime_checker(0));
        assert_eq!(None, prime_checker(1));
    }
    #[test]
    fn prime_check_big() {
        assert_eq!(Some(Err(PrimeErr::Divider(7993))), prime_checker(63888049));
        assert_eq!(Some(Ok(2147483647)), prime_checker(2147483647));
    }
}