
use std::ops::Add;
use std::cmp::PartialOrd;

pub struct Triangle<T> {
    a: T,
    b: T,
    c: T,
}

impl<T> Triangle<T>
    where T: PartialOrd + Add<Output = T> + Copy + Default
{
    // use default as zero
    pub fn build(slice: [T; 3]) -> Result<Triangle<T>, String> {
        let (a, b, c) = (slice[0], slice[1], slice[2]);
        if slice.iter().any(|&s| s == T::default()) || a + b < c || a + c < b || b + c < a {
            Err(String::from("not a valid trangle"))
        } else {
            Ok(Triangle { a: a, b: b, c: c })
        }
    }
    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.b == self.c
    }

    pub fn is_isosceles(&self) -> bool {
        !self.is_equilateral() && (self.a == self.b || self.b == self.c || self.a == self.c)
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_equilateral() && !self.is_isosceles()
    }
}
