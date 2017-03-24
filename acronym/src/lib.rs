pub fn abbreviate(phase: &str) -> String {
    phase.chars()
        .fold((String::new(), ' '), |(mut acc, last), c| {
            if c.is_alphabetic() &&
               (!last.is_alphabetic() || (c.is_uppercase() && !last.is_uppercase())) {
                acc.push(c)
            }
            (acc, c)
        })
        .0
        .to_uppercase()
}
