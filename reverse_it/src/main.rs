pub fn reverse_it(v: i32) -> String {
    let x = v.abs().to_string();

    let mut rev=x.clone().chars().rev().collect::<String>();
    rev.push_str(&x);
    if v< 0 {
         let mut minus = String::from("-");
         minus.push_str(&rev);
         return minus
    }
    return rev.to_string();
}

fn main() {
    println!("{}", reverse_it(123));
    println!("{}", reverse_it(-123));
}

#[test]
fn reverse_it_test() {
    assert_eq!("321123", &reverse_it(123));
    assert_eq!("987654321123456789", &reverse_it(123456789));
    assert_eq!("00", &reverse_it(0));
    assert_eq!("-321123", &reverse_it(-123));
    assert_eq!("11", &reverse_it(1));
}

