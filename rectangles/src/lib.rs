type Point = (usize, usize);

fn valid_rect(up_left: Point, right_down: Point, data: &Vec<Vec<char>>) -> bool {
    up_left.0 < right_down.0 && up_left.1 < right_down.1 &&
    data[up_left.0][right_down.1] == '+' && data[right_down.0][up_left.1] == '+' &&
    (up_left.1..right_down.1).map(|y| data[up_left.0][y]).all(|c| c == '+' || c == '-') &&
    (up_left.1..right_down.1).map(|y| data[right_down.0][y]).all(|c| c == '+' || c == '-') &&
    (up_left.0..right_down.0).map(|x| data[x][up_left.1]).all(|c| c == '+' || c == '|') &&
    (up_left.0..right_down.0).map(|x| data[x][right_down.1]).all(|c| c == '+' || c == '|')
}

pub fn count(lines: &[&str]) -> usize {
    let corners: Vec<Point> = lines.iter()
        .enumerate()
        .flat_map(|(x_idx, &row)| {
            row.chars()
                .enumerate()
                .filter(|&(_, cell)| cell == '+')
                .map(move |(y_idx, _)| (x_idx, y_idx))
        })
        .collect();

    let data: Vec<Vec<char>> = lines.iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    (0..corners.len())
        .flat_map(|i| (i + 1..corners.len()).map(move |j| (i, j)))
        .map(|(i, j)| (corners[i], corners[j]))
        .filter(|&(up, down)| valid_rect(up, down, &data))
        .count()
}
