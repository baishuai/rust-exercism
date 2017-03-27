use std::collections::BTreeMap;
pub struct Proteins<'a> {
    pairs: BTreeMap<&'a str, &'a str>,
}

pub fn parse(vec: Vec<(&'static str, &'static str)>) -> Proteins {
    Proteins { pairs: vec.iter().cloned().collect() }
}

fn compress(name: &str) -> String {
    name.chars()
        .map(|c| {
            match c {
                'H' | 'Y' => 'T',
                'M' => 'C',
                'N' | 'R' => 'A',
                x => x,
            }
        })
        .collect()
}

impl<'a> Proteins<'a> {
    pub fn name_for(&self, rna: &str) -> Result<&str, String> {
        self.pairs
            .get(rna)
            .or_else(|| self.pairs.get(compress(rna).as_str()))
            .cloned()
            .ok_or(String::from("invalid rna"))
    }
}