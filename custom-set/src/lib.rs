#[derive(Debug)]
pub struct CustomSet<T> {
    items: Vec<T>,
}

impl<T: PartialEq> PartialEq for CustomSet<T> {
    fn eq(&self, other: &CustomSet<T>) -> bool {
        self.is_subset(other) && other.is_subset(self)
    }
}

impl<T: PartialEq> CustomSet<T> {
    pub fn new(items: Vec<T>) -> CustomSet<T> {
        let mut vec = Vec::with_capacity(items.len());
        for item in items {
            if !vec.contains(&item) {
                vec.push(item);
            }
        }
        CustomSet { items: vec }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn add(&mut self, item: T) {
        if !self.items.contains(&item) {
            self.items.push(item);
        }
    }

    pub fn delete(&mut self, item: &T) {
        if let Some(i) = self.items.iter().position(|n| item == n) {
            self.items.remove(i);
        }
    }

    pub fn is_subset(&self, other: &CustomSet<T>) -> bool {
        self.items.iter().all(|i| other.items.contains(i))
    }

    pub fn is_disjoint(&self, other: &CustomSet<T>) -> bool {
        !self.items.iter().any(|i| other.items.contains(&i))
    }
}

impl<T: PartialEq + Clone> CustomSet<T> {
    pub fn difference(&self, other: &CustomSet<T>) -> CustomSet<T> {
        let diff = self.items.iter().filter(|i| !other.items.contains(i)).cloned().collect();
        CustomSet { items: diff }
    }

    pub fn symmetric_difference(&self, other: &CustomSet<T>) -> CustomSet<T> {
        let d1 = self.difference(other);
        let d2 = other.difference(self);
        d1.union(&d2)
    }

    pub fn contains(&self, item: &T) -> bool {
        self.items.contains(item)
    }

    pub fn union(&self, other: &CustomSet<T>) -> CustomSet<T> {
        CustomSet::new(self.items.iter().chain(other.items.iter()).cloned().collect())
    }

    pub fn intersection(&self, other: &CustomSet<T>) -> CustomSet<T> {
        let mutual = self.items.iter().filter(|i| other.items.contains(i)).cloned().collect();
        CustomSet { items: mutual }
    }
}