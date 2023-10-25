use std::io::{self, Write};

fn brain_fuck(s: &str) {
    let mut tape = [0u8; 2048];
    let mut ptr = 0usize;
    let mut pc = 0usize;

    while pc < s.len() {
        match s.chars().nth(pc).unwrap() {
            '>' => ptr += 1,
            '<' => ptr -= 1,
            '+' => tape[ptr] += 1,
            '-' => tape[ptr] -= 1,
            '.' => print!("{}", tape[ptr] as char),
            '[' => {
                if tape[ptr] == 0 {
                    let mut loop_depth = 1;
                    while loop_depth > 0 {
                        pc += 1;
                        match s.chars().nth(pc).unwrap() {
                            '[' => loop_depth += 1,
                            ']' => loop_depth -= 1,
                            _ => (),
                        }
                    }
                }
            }
            ']' => {
                let mut loop_depth = 1;
                while loop_depth > 0 {
                    pc -= 1;
                    match s.chars().nth(pc).unwrap() {
                        '[' => loop_depth -= 1,
                        ']' => loop_depth += 1,
                        _ => (),
                    }
                }
                pc -= 1;
            }
            _ => (),
        }
        pc += 1;
    }
}



fn main() {
    // let mut buffer = String::new();
    // io::stdin().read_to_string(&mut buffer).unwrap();
    // brain_fuck(&buffer.trim());
    // // std::io::stdout().flush().unwrap();
    let args: Vec<String> = std::env::args().collect();

    // Flush the output buffer immediately
    let _ = io::stdout().flush();

    brain_fuck(&args[1]);
}
#[cfg(test)]
mod tests {
    use std::process::Command;

    // const MANIFEST_PATH: &str = "../../solutions/brain_fuck/Cargo.toml";

    fn run(s: &str) -> String {
        let output = Command::new("cargo")
            // .arg("run")
            // .arg("--manifest-path")
            // .arg(MANIFEST_PATH)
            .arg(s)
            .output()
            .expect("Failed to execute command");

        String::from_utf8(output.stdout).unwrap()
    }

    #[test]
    fn nothing_passed() {
        assert_eq!("", run(""));
    }

    #[test]
    fn single_chars() {
        assert_eq!(
            "a",
            run("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>>---.")
        );
        assert_eq!(
            "S",
            run("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>+++++++++++++.")
        );
        assert_eq!(
            "7",
            run("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>---------------.")
        );
    }
    #[test]
    fn phrases() {
        assert_eq!(
            "Wow",
            run("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>>-------------.++++++++++++++++++++++++.++++++++.")
        );
        assert_eq!(
            "Good job!",
            run("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>+.>+++++++++++..-----------.<<++.>>++++++.+++++.-------------.<<+.")
        );
    }

    #[test]
    fn with_characters_in_middle() {
        assert_eq!("keep going", run("++++++++++[>+>ke+++>+++++++>++++++++++<<<<-]>>>>+++++++e.------p..+++++++++++.<<++.>g>---------.+o+++++++.------i.+++++.-n------.g"));
    }

    #[test]
    fn big_test() {
        assert_eq!(
            "3, 2, 1... Happy New Year",
            run("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>-------------------.-------.<++.>++++++.------.<.>+++++.---...<.>++++++++++++++++++++++++++.>---.+++++++++++++++..+++++++++.<<.>++++++.>--------------------.++++++++++++++++++.<<.>+++++++++++.++++++++++++.----.>-----.")
        );
        assert_eq!(
            "To be or not be, that is the question!", 
            run("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>++++++++++++++.>+++++++++++.<<++.>>-------------.+++.<<.>>++++++++++.+++.<<.>>----.+.+++++.<<.>++++++++++++++.+++.<++++++++++++.------------.>>.<+++.-------.>.<<.>++++++++.>-.<<.>>+.<-.---.<.>>---.++++.<.>--.+.<++++.>-----.-.<<+.")
        );
    }
}
