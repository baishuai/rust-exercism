use std::collections::HashMap;

pub fn word_count(phrase: &str) -> HashMap<String, u32> {
    let mut map: HashMap<String, u32> = HashMap::new();
    for w in phrase.to_lowercase()
        .split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()))
        .filter(|w| w.len() > 0) {
        *map.entry(String::from(w)).or_insert(0) += 1;
    }
    map
}