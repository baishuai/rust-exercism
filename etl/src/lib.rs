use std::collections::BTreeMap;

pub fn transform(input: &BTreeMap<i32, Vec<String>>) -> BTreeMap<String, i32> {
    input.iter()
        .flat_map(|(&s, list)| list.iter().map(move |word| (word.to_lowercase(), s)))
        .collect()
}
