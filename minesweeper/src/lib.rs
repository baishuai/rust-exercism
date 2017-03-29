pub fn annotate(cleaned_strs: &[&str]) -> Vec<String> {
    let data: Vec<Vec<bool>> = cleaned_strs.into_iter()
        .map(|s| s.chars().map(|c| c == '*').collect())
        .collect();
    let sizex = data.len() as isize;
    let sizey = data[0].len() as isize;

    let is_mine = |x, y| {
        if x < 0 || x >= sizex || y < 0 || y >= sizey {
            false
        } else {
            data[x as usize][y as usize]
        }
    };

    let offs = (-1..2)
        .flat_map(|i| {
            (-1..2)
                .map(|j| (i, j))
                .collect::<Vec<(isize, isize)>>()
        })
        .collect::<Vec<_>>();

    (0..sizex)
        .map(|x| {
            (0..sizey)
                .map(|y| if is_mine(x, y) {
                    '*'
                } else {
                    match offs.iter()
                        .filter(|&&(a, b)| is_mine(x + a, y + b))
                        .count() {
                        0 => ' ',
                        x => (x as u8 + '0' as u8) as char,
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
}
