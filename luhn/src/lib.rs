pub fn is_valid(num: &str) -> bool {
    if num.chars()
        .filter(|c| !c.is_whitespace())
        .any(|c| !c.is_numeric()) ||
       num.chars()
        .filter(|c| !c.is_whitespace())
        .take(2)
        .count() < 2 {
        return false;
    }
    num.chars()
        .filter(|c| !c.is_whitespace())
        .rev()
        .enumerate()
        .map(|(i, c)| {
            println!("{:?},{:?}", i, (c as u32 - '0' as u32));

            let v = ((c as u32) - '0' as u32) * 2;
            if i % 2 == 1 {
                println!("{:?}", v / 2);
                if v > 9 { v - 9 } else { v }
            } else {
                v / 2
            }
        })
        .sum::<u32>() % 10 == 0
}