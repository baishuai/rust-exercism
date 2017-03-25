#[allow(unused_variables)]

use std::collections::{BTreeMap, BinaryHeap};

pub struct School {
    gs: BTreeMap<u32, BinaryHeap<String>>,
}

impl School {
    pub fn new() -> School {
        School { gs: BTreeMap::new() }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.gs.entry(grade).or_insert(BinaryHeap::new()).push(String::from(student))
    }

    pub fn grades(&self) -> Vec<u32> {
        self.gs.keys().cloned().collect::<Vec<_>>()
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.gs.get(&grade).map(|students| students.clone().into_sorted_vec())
    }
}
