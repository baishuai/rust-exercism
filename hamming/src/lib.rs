

pub fn hamming_distance(a: &str, b: &str) -> Result<usize, ()> {
    if a.len() != b.len() {
        Err(())
    } else {
        Ok(a.chars()
            .into_iter()
            .zip(b.chars().into_iter())
            .filter(|&(ac, bc)| ac != bc)
            .count())
    }
}