pub fn sum_of_squares(n: i32) -> i32 {
    (1..n + 1).map(|x| x * x).sum()
}

pub fn square_of_sum(n: i32) -> i32 {
    (1..n + 1).sum::<i32>().pow(2)
}


pub fn difference(n: i32) -> i32 {
    square_of_sum(n) - sum_of_squares(n)
}