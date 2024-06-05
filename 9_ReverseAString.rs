fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let s = "hello";
    println!("The reverse of '{}' is '{}'", s, reverse_string(s));
}
