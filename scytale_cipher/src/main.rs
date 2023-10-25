pub fn scytale_cipher(message: String, i: u32) -> String {
    let s: Vec<char> = message.chars().collect();
    let range: usize = ((s.len() as f64) / (i as f64)).ceil() as usize;
    let mut result: String = String::new();
    let mut index: usize = 0;

    for j in 0..i as usize {
        for _ in 0..range {
            let place = j + i as usize * index;

            if place >= s.len() {
                result += " ";
            } else {
                result += &s[place].to_string();
            }

            index += 1;
        }
        index = 0;
    }

    return result.trim().to_string();
}

fn main() {
    println!("\"scytale Code\" size=6 -> {:?}", scytale_cipher(String::from("scytale Code"), 6));
    println!("\"scytale Code\" size=8 -> {:?}", scytale_cipher(String::from("scytale Code"), 8));
}
