extern crate threadpool;

use std::collections::HashMap;
use std::sync::mpsc;
use threadpool::ThreadPool;

pub fn frequency(text: &[&'static str], num_threads: usize) -> HashMap<char, usize> {
    let pool = ThreadPool::new(num_threads);
    let (tx, rx) = mpsc::channel();

    for &line in text {
        let tx = tx.clone();
        pool.execute(move || {
            tx.send(calculate(line)).unwrap();
        })
    }

    rx.iter()
        .take(text.len())
        .fold(HashMap::new(), |mut acc, h| {
            for (c, n) in h.into_iter() {
                *acc.entry(c).or_insert(0) += n;
            }
            acc
        })
}

fn calculate(line: &str) -> HashMap<char, usize> {
    line.to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        })
}