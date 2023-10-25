use std::vec;

fn main() {
    let args_list: Vec<String> = std::env::args().collect();

    if args_list.len() != 2 {
        println!("Error");
        return;
    }

    let mut stack: Vec<isize> = vec![];
    // let mut err = false;
    let mut error_occurred = false;

    // loop through each item in the split array
    'outer: for token in args_list[1].split_whitespace() {
        let parsed_token = token.parse();
        match parsed_token {
            // if is integer, push to stack.
            Ok(tok) => {
                stack.push(tok);
            }
            Err(_) => {
                // check if length is greater than 2 before popping.
                if stack.len() < 2 {
                    println!("Error");
                    return;
                }

                // else,
                let last_in = match stack.pop() {
                    Some(value) => Some(value),
                    None => None,
                };

                let second_last_in = match stack.pop() {
                    Some(value) => Some(value),
                    None => None,
                };

                // handle token
                match token {
                    "+" => {
                        if let (Some(a), Some(b)) = (second_last_in, last_in) {
                            stack.push(a + b);
                        } else {
                            error_occurred = true;
                            break 'outer;
                        }
                    }
                    "-" => {
                        if let (Some(a), Some(b)) = (second_last_in, last_in) {
                            stack.push(a - b);
                        } else {
                            error_occurred = true;
                            break 'outer;
                        }
                    }
                    "*" => {
                        if let (Some(a), Some(b)) = (second_last_in, last_in) {
                            stack.push(a * b);
                        } else {
                            error_occurred = true;
                            break 'outer;
                        }
                    }
                    "/" => {
                        if let (Some(a), Some(b)) = (second_last_in, last_in) {
                            stack.push(a / b);
                        } else {
                            error_occurred = true;
                            break 'outer;
                        }
                    }
                    "%" => {
                        if let (Some(a), Some(b)) = (second_last_in, last_in) {
                            stack.push(a % b);
                        } else {
                            error_occurred = true;
                            break 'outer;
                        }
                    }
                    _ => {
                        error_occurred = true;
                        break 'outer;
                    }
                }
            } // operator
        };
    }

    if error_occurred || stack.len() != 1 {
        println!("Error");
    } else {
        println!("{}", stack[0]);
    }
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    const MANIFEST_PATH: &str = "../../solutions/rpn/Cargo.toml";

    fn run(s: &str) -> String {
        let output = Command::new("cargo")
            .arg("run")
            .arg("--manifest-path")
            .arg(MANIFEST_PATH)
            .arg(s)
            .output()
            .expect("Failed to execute command");

        String::from_utf8(output.stdout).unwrap()
    }

    #[test]
    fn error_tests() {
        assert_eq!("Error\n", run("21 3 2 % 2 3 2 *"));
        assert_eq!("Error\n", run("1 2 3 4 +"));
        assert_eq!("Error\n", run("324   +    1 - 23 "));
        assert_eq!("Error\n", run("32   / 22"));
        assert_eq!("Error\n", run("88 67 dks -"));
    }

    #[test]
    fn simple_tests() {
        assert_eq!("33\n", run("11 22 +"));
        assert_eq!("72\n", run("11016 153 /"));
        assert_eq!("1140\n", run("15 76 *"));
        assert_eq!("-78539698\n", run("23491234 102030932 -"));
    }

    #[test]
    fn complex_tests() {
        assert_eq!("10\n", run("1 2 * 3 * 4 +"));
        assert_eq!("2\n", run("3 1 2 * * 4 %"));
        assert_eq!("0\n", run("5 10 2 / - 50 *"));
    }

    #[test]
    fn with_spaces() {
        assert_eq!("44\n", run("299   255 %"));
        assert_eq!("1\n", run("     1      3 * 2 -"));
    }
}