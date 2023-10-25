pub fn rotate(input: &str, key: i8) -> String {
    let mut result = String::new();
    let input_chars: Vec<char> = input.chars().collect();
    let alphabet_len = 26;

    for ch in input_chars {
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

    println!("The letter \"a\" becomes: {}", rotate("a", 26));
    println!("The letter \"m\" becomes: {}", rotate("m", 0));
    println!("The letter \"m\" becomes: {}", rotate("m", 13));
    println!("The letter \"a\" becomes: {}", rotate("a", 15));
    println!("The word \"MISS\" becomes: {}", rotate("MISS", 5));
    println!(
        "The decoded message is: {}",
        rotate("Gur svir obkvat jvmneqf whzc dhvpxyl.", 13)
    );
    println!(
        "The decoded message is: {}",
        rotate("Mtb vznhpqd ifky ozrunsl ejgwfx ajc", 5)
    );
    println!(
        "Your cypher wil be: {}",
        rotate("Testing with numbers 1 2 3", 4)
    );
    println!("Your cypher wil be: {}", rotate("Testing", -14));
    println!("The letter \"a\" becomes: {}", rotate("a", -1));
}