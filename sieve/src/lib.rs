pub fn primes_up_to(upper: usize) -> Vec<u32> {
    (2..)
        .take(upper - 1)
        .filter(|&n| (2..(n as f32).sqrt() as u32 + 1).all(|i| n % i != 0))
        .collect()
}
