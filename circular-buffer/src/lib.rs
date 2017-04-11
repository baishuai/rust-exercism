use std::iter;
use std::fmt::Debug;

#[derive(Debug)]
pub struct CircularBuffer<T> {
    data: Vec<T>,
    cap: usize,
    len: usize,
    lo: usize,
    hi: usize,
}

#[derive(Debug,PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}


impl<T: PartialEq + Default + Clone + Debug> CircularBuffer<T> {
    pub fn new(size: usize) -> CircularBuffer<T> {
        CircularBuffer {
            data: iter::repeat(T::default()).take(size).collect(),
            cap: size,
            len: 0,
            lo: 0,
            hi: 0,
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        println!("{:?}", self);
        if self.len == 0 {
            Err(Error::EmptyBuffer)
        } else {
            self.len -= 1;
            let lo = self.lo;
            self.lo = (self.lo + 1) % self.cap;
            Ok(self.data[lo].clone())
        }
    }

    pub fn write(&mut self, v: T) -> Result<(), Error> {
        if self.len == self.cap {
            Err(Error::FullBuffer)
        } else {
            self.len += 1;
            self.data[self.hi] = v;
            self.hi = (self.hi + 1) % self.cap;
            Ok(())
        }
    }

    pub fn overwrite(&mut self, v: T) {
        if self.len == self.cap {
            self.lo += 1;
        } else {
            self.len += 1;
        }
        self.data[self.hi] = v;
        self.hi = (self.hi + 1) % self.cap;
    }

    pub fn clear(&mut self) {
        self.lo = 0;
        self.hi = 0;
        self.len = 0;
    }
}