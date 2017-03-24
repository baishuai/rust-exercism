pub fn lsp(num: &str, window: usize) -> Result<u32, String> {
    if num.len() < window {
        Err(String::from("length is 0"))
    } else if num.chars().any(|c| !c.is_digit(10)) {
        Err(String::from("is not digit"))
    } else if window == 0 {
        Ok(1)
    } else {
        Ok(num.chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>()
            .windows(window)
            .map(|w| w.iter().product::<u32>())
            .max()
            .unwrap_or(1))
    }
}