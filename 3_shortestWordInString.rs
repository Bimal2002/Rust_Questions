fn shortest_word(s: &str) -> &str {
    s.split_whitespace().min_by_key(|word| word.len()).unwrap_or("")
}

fn main() {
    let sentence = "The quick brown fox jumps over the lazy dog";
    println!("The shortest word is '{}'", shortest_word(sentence));
}
