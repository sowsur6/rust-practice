fn main() {
    // Get the command-line arguments, skipping the first argument (the program name)
    let args: Vec<String> = std::env::args().skip(1).collect();

    for arg in args {
        if is_well_bracketed(&arg) {
            println!("OK");
        } else {
            println!("Error");
        }
    }
}

fn is_well_bracketed(s: &str) -> bool {
    let mut stack = Vec::new();

    for c in s.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' => if stack.pop() != Some('(') { return false; },
            ']' => if stack.pop() != Some('[') { return false; },
            '}' => if stack.pop() != Some('{') { return false; },
            _ => {},
        }
    }

    stack.is_empty()
}



#[cfg(test)]
mod tests {

    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    use std::process::{Command, Output};

    const MANIFEST_PATH: &str = "../../solutions/brackets_matching/Cargo.toml";

    fn run(s: Vec<&str>) -> Output {
        Command::new("cargo")
            .arg("run")
            .arg("--manifest-path")
            .arg(MANIFEST_PATH)
            .args(s.iter())
            .output()
            .expect("Failed to execute command")
    }

    #[test]
    fn random_tests() {
        fn random_alnum() -> String {
            thread_rng()
                .sample_iter(&Alphanumeric)
                .take(30)
                .map(char::from)
                .collect()
        }
        let mut args = vec![
            String::from("(johndoe)"),
            String::from("()"),
            String::from("([])"),
            String::from("{2*[d - 3]/(12)}"),
        ];

        for _ in 0..3 {
            args.push(format!("({:?})", &random_alnum()).as_str());
            args.push(format!("[{:?}]", &random_alnum()).as_str());
            args.push(format!("{}{:?}{}", "{", &random_alnum(), "}").as_str());
        }

        for v in args.iter() {
            let output = run(vec![v]);
            assert_eq!(String::from_utf8(output.stdout).unwrap(), "OK\n");
        }
    }

    #[test]
    fn tests_both() {
        struct Test<'a> {
            arguments: ([&'a str; 2], &'a str),
        }

        let arr: [Test; 3] = [
            Test {
                arguments: (
                    ["", "{[(0 + 0)(1 + 1)](3*(-1)){()}}"].to_vec(),
                    "OK\nOK\n".to_string().as_str(),
                ),
            },
            Test {
                arguments: (
                    ["{][]}", "{3*[21/(12+ 23)]}"].to_vec(),
                    "Error\nOK\n".to_string().as_str(),
                ),
            },
            Test {
                arguments: (
                    ["{([)])}", "{{{something }- [something]}}"].to_vec(),
                    "Error\nOK\n".to_string().as_str(),
                ),
            },
        ];

        for t in arr.iter() {
            let output = run(t.arguments.0.to_vec());
            assert_eq!(String::from_utf8_lossy(&output.stdout), t.arguments.1);
        }
    }

    #[test]
    fn tests_with_nothing() {
        let output = run(vec![]);
        assert_eq!(String::from_utf8_lossy(&output.stdout), "");
    }
}
