// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

pub fn convert(input: &str) -> Result<String, ()> {
    let lines_count = input.lines().count();
    if lines_count % 4 != 0 || input.lines().any(|l| l.len() % 3 != 0) {
        return Err(());
    }
    let mut iter = input.lines();
    let result = (0..(lines_count / 4))
        .map(|_| {
            handle_4line(&iter.by_ref().take(4).map(|l| String::from(l) + "\n").collect::<String>())
        })
        .collect::<Vec<String>>()
        .join(",");
    Ok(result)
}


fn handle_4line(input: &str) -> String {
    let line_lines: Vec<Vec<char>> = input.lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    let mut result = String::new();
    for char_nr in 0..(line_lines[0].len() / 3) {
        let mut curr_char = String::new();
        for i in 0..4 {
            for j in 0..3 {
                curr_char.push(line_lines[i][char_nr * 3 + j]);
            }
        }
        result.push(handle_char(&curr_char));
    }
    result
}

fn handle_char(input: &str) -> char {
    match input {
        " _ | ||_|   " => '0',
        "     |  |   " => '1',
        " _  _||_    " => '2',
        " _  _| _|   " => '3',
        "   |_|  |   " => '4',
        " _ |_  _|   " => '5',
        " _ |_ |_|   " => '6',
        " _   |  |   " => '7',
        " _ |_||_|   " => '8',
        " _ |_| _|   " => '9',
        _ => '?',
    }
}