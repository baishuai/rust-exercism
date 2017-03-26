use std::collections::BinaryHeap;

pub fn anagrams_for(word: &'static str, dict: &[&'static str]) -> Vec<&'static str> {
    dict.iter()
        .filter(|opt| {
            let opt = opt.to_lowercase();
            let word = word.to_lowercase();
            opt != word &&
            word.chars().collect::<BinaryHeap<_>>().into_sorted_vec() ==
            opt.chars().collect::<BinaryHeap<_>>().into_sorted_vec()
        })
        .map(|&s| s)
        .collect::<Vec<_>>()
}
