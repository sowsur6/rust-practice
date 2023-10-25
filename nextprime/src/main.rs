

// pub fn next_prime(nbr: u32) -> u32 {
//     let mut i = nbr + 1;
//     loop {
//         let mut is_prime = true;
//         for j in 2..=i / 2 {
//             if i % j == 0 {
//                 is_prime = false;
//                 break;
//             }
//         }
//         if is_prime {
//             return i;
//         }
//         i += 1;
//     }
// }
pub fn next_prime(nbr: u64) -> u64 {
    if nbr < 2 {
        return 2;
    }
    if nbr == 2 {
        return 2;
    }
    let mut num = if nbr % 2 == 0 { nbr + 1 } else { nbr };
    loop {
        let mut is_prime = true;
        let limit = (num as f64).sqrt() as u64 + 1;
        for i in (3..=limit).step_by(2) {
            if num % i == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            return num;
        }
        num += 2;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_tests() {
        assert_eq!(2, next_prime(0));
        assert_eq!(5, next_prime(5));
        assert_eq!(37, next_prime(32));
        assert_eq!(641, next_prime(633));
        assert_eq!(478157, next_prime(478152));
    }
}



fn main() {
    println!("The next prime after 1 is: {}", next_prime(1));
    println!("The next prime after 11 is: {}", next_prime(11));
}