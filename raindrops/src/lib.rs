pub fn raindrops(num: i32) -> String {
    let mut string = String::new();
    // 3,5,7
    let factors = [(3, "Pling"), (5, "Plang"), (7, "Plong")];
    for &(n, word) in &factors {
        if num % n == 0 {
            string.push_str(word);
        }
    }
    if string.is_empty() {
        string.push_str(&num.to_string());
    }
    string
}