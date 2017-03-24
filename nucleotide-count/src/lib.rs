use std::collections::HashMap;

pub fn count(nuc: char, dna: &str) -> Result<usize, String> {
    if dna.chars()
        .filter(|&c| c != 'A' && c != 'T' && c != 'C' && c != 'G')
        .count() > 0 {
        return Err(String::from("Error nucleotide in dna"));
    }
    match nuc {
        'A' | 'T' | 'C' | 'G' => {
            Ok(dna.chars()
                .filter(|&c| c == nuc)
                .count())
        }
        _ => Err(String::from("Error nucleotide")),
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, String> {
    let mut map = HashMap::new();
    for &c in &['A', 'T', 'C', 'G'] {
        let cnt = count(c, dna)?;
        map.insert(c, cnt);
    }
    Ok(map)
}
