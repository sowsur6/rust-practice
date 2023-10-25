pub fn rot21(input: &str) -> String {
    let mut result = String::new();
    let alphabet_len = 26;
    let key = 21;

    for ch in input.chars() {
        if ch.is_ascii_alphabetic() {
            let ch_upper = ch.to_ascii_uppercase();
            let ch_offset = ch_upper as i8 - 'A' as i8;
            let rotated_offset = (ch_offset + key) % alphabet_len;
            let rotated_char = ((rotated_offset + alphabet_len) % alphabet_len) as u8 + 'A' as u8;
            let rotated_ch = rotated_char as char;
            result.push(if ch.is_uppercase() {
                rotated_ch
            } else {
                rotated_ch.to_ascii_lowercase()
            });
        } else {
            result.push(ch);
        }
    }

    result
}

fn main() {
    println!("The letter \"a\" becomes: {}", rot21("a"));
    println!("The letter \"m\" becomes: {}", rot21("m"));
    println!("The word \"MISS\" becomes: {}", rot21("MISS?"));
    println!("Your cypher wil be: {}", rot21("Testing numbers 1 2 3"));
    println!("Your cypher wil be: {}", rot21("rot21 works!"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rot21_multiple_cases() {
        assert_eq!("ocdn dn v ozno", rot21("this is a test"));
        assert_eq!("mviyjh ndhkgz rjmyn", rot21("random simple words"));
        assert_eq!(
            "ojj  hpxc    nkvxzn      rjmfn",
            rot21("too  much    spaces      works")
        );
        assert_eq!("mv😋w", rot21("ra😋b"));
        assert_eq!("12Â nãj ábpv", rot21("12Â são água"));

        assert_eq!("VWXY", rot21("ABCD"));
        assert_eq!("GJJFDIB BJJY", rot21("LOOKING GOOD"));
        assert_eq!("WTZ", rot21("BYE"));
    }
}