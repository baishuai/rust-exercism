
#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(sl: &[T], sr: &[T]) -> Comparison {

    let (maybe, short, long) = if sl.len() > sr.len() {
        (Comparison::Superlist, sr, sl)
    } else if sl.len() == sr.len() {
        (Comparison::Equal, sl, sr)
    } else {
        (Comparison::Sublist, sl, sr)
    };

    let diff = long.len() - short.len();
    (0..diff + 1)
        .find(|&i| {
            short.iter()
                .zip(long.iter().skip(i))
                .all(|(l, r)| l == r)
        })
        .map_or(Comparison::Unequal, |_| maybe)
}