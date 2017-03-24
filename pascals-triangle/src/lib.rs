pub struct PascalsTriangle {
    tri: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut t: Vec<Vec<u32>> = Vec::new();
        for i in 0..row_count as usize {
            let mut r: Vec<u32> = Vec::new();
            r.push(1);
            for j in 1..i {
                r.push(t[i - 1][j - 1] + t[i - 1][j]);
            }
            if i > 0 {
                r.push(1);
            }
            t.push(r);
        }
        PascalsTriangle { tri: t }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.tri.clone()
    }
}
