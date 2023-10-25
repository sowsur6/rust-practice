pub fn negative_spell(n: i64) -> String {
    // Check if the number is positive, return error message if so
    if n >= 1 {
        return String::from("error: positive number");
    }else if n == 0{
        return String::from("zero");
    }

    // Define mapping of digit values to corresponding word spellings
    let digit_spellings: [&str; 20] = [
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen",
    ];

    // Define mapping of tens place values to corresponding word spellings
    let tens_spellings: [&str; 10] = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    // Check if the number is -1 million
    if n == -1_000_000 {
        return String::from("minus one million");
    }

    // Create a StringBuilder to build the spelled out negative number
    let mut result = String::from("minus ");

    // Convert the number to positive and process the digits
    let num = n.abs();
    let thousands = num / 1000;
    let hundreds = (num / 100) % 10;
    let tens = (num / 10) % 10;
    let ones = num % 10;

    if thousands > 0 {
        let hundreds = (thousands / 100) % 10;
        let tens = (thousands / 10) % 10;
        let ones = thousands % 10;
        if hundreds > 0 {
        result.push_str(digit_spellings[hundreds as usize]);
        result.push_str(" hundred ");
        }
    if tens > 0 {
        if tens == 1 {
            result.push_str(digit_spellings[10 + ones as usize]);
           result.push_str(" thousand ");
        } else {
            result.push_str(tens_spellings[tens as usize]);
            if ones > 0 {
                result.push('-');
            } else {
                result.push(' ');
            }
        }
    }

    if ones > 0 {
        result.push_str(digit_spellings[ones as usize]);
        result.push_str(" thousand ");
    }

    }

    if hundreds > 0 {
        result.push_str(digit_spellings[hundreds as usize]);
        result.push_str(" hundred ");
    }

    if tens > 0 {
        if tens == 1 {
            result.push_str(digit_spellings[10 + ones as usize]);
            return result;
        } else {
            result.push_str(tens_spellings[tens as usize]);
            if ones > 0 {
                result.push('-');
            } else {
                result.push(' ');
            }
        }
    }

    if ones > 0 {
        result.push_str(digit_spellings[ones as usize]);
    }

    result.trim().to_string() // Return the spelled out negative number
}
fn main() {
    println!("{}", negative_spell(-1234));
    println!("{}", negative_spell(100));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_numbers() {
        assert_eq!(negative_spell(0), String::from("zero"));
        assert_eq!(negative_spell(-1), String::from("minus one"));
        assert_eq!(negative_spell(-14), String::from("minus fourteen"));
        assert_eq!(negative_spell(-20), String::from("minus twenty"));
        assert_eq!(negative_spell(-22), String::from("minus twenty-two"));
        assert_eq!(negative_spell(-101), String::from("minus one hundred one"));
        assert_eq!(
            negative_spell(-120),
            String::from("minus one hundred twenty")
        );
        assert_eq!(
            negative_spell(-123),
            String::from("minus one hundred twenty-three")
        );
    }
    #[test]
    fn test_medium_numbers() {
        assert_eq!(negative_spell(-1000), String::from("minus one thousand"));
        assert_eq!(
            negative_spell(-1055),
            String::from("minus one thousand fifty-five")
        );
        assert_eq!(
            negative_spell(-1234),
            String::from("minus one thousand two hundred thirty-four")
        );
        assert_eq!(
            negative_spell(-10123),
            String::from("minus ten thousand one hundred twenty-three")
        );
    }
    #[test]
    fn test_long_numbers() {
        assert_eq!(
            negative_spell(-910112),
            String::from("minus nine hundred ten thousand one hundred twelve")
        );
        assert_eq!(
            negative_spell(-651123),
            String::from("minus six hundred fifty-one thousand one hundred twenty-three")
        );

        assert_eq!(
            negative_spell(-810000),
            String::from("minus eight hundred ten thousand")
        );
        assert_eq!(negative_spell(-1000000), String::from("minus one million"));
    }
    #[test]
    fn test_invalid_numbers() {
        assert_eq!(negative_spell(1), String::from("error: positive number"));
        assert_eq!(
            negative_spell(2390904),
            String::from("error: positive number")
        );
    }
}