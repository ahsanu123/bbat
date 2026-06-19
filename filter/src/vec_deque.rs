use std::collections::VecDeque;

pub struct FiniteBuffer<T: Default + Clone> {
    buffer: VecDeque<T>,
    capacity: usize,
}

impl<T: Default + Clone> FiniteBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        FiniteBuffer {
            buffer: VecDeque::from(vec![T::default(); capacity]),
            capacity,
        }
    }

    pub fn push(&mut self, item: T) {
        if self.buffer.len() == self.capacity {
            self.buffer.pop_back();
        }
        self.buffer.push_front(item);
    }

    pub fn get(&self, index: usize) -> &T {
        self.buffer.get(index).unwrap()
    }
}

pub type SampleBuffer = FiniteBuffer<f64>;
