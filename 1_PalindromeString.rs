fn is_palindrome(s: &str) -> bool {
    let s: Vec<char> = s.chars().collect();
    let n = s.len();
    for i in 0..n / 2 {
        if s[i] != s[n - i - 1] {
            return false;
        }
    }
    true
}

fn main() {
    let word = "racecar";
    println!("Is '{}' a palindrome? {}", word, is_palindrome(word));
}
