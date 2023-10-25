fn count_factorial_steps(factorial: u64) -> u64 {
    if factorial <= 1 {
        return 0;
    }

    let mut steps = 0;
    let mut result = 1;

    for i in 1..=factorial {
            result *= i;
            steps += 1;
            if result == factorial {
                return steps;
            }
            if result > factorial {
            return 0;
            }
    }

    return 0;
}

fn main() {
    println!("The factorial steps of 720 = {}", count_factorial_steps(720));
    println!("The factorial steps of 13 = {}", count_factorial_steps(13));
    println!("The factorial steps of 6 = {}", count_factorial_steps(6));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_factorial_steps_edge_cases() {
        assert_eq!(0, count_factorial_steps(0));
        assert_eq!(0, count_factorial_steps(1));
        assert_eq!(0, count_factorial_steps(123));
    }
    #[test]
    fn count_factorial_steps_normal_cases() {
        assert_eq!(6, count_factorial_steps(720));
        assert_eq!(10, count_factorial_steps(3628800));
    }
}

