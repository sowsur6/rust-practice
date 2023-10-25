
pub fn reverse_it(v: i32) -> String {
    let  s = v.abs().to_string();
    let rev = s.chars().rev().collect::<String>();
    let result = if v < 0 {
        format!("-{}{}", rev, (-v).to_string())
    } else {
        format!("{}{}", rev, s)
    };
    result
}


fn main() {
    println!("{}", reverse_it(123));
    println!("{}", reverse_it(-123));
}
