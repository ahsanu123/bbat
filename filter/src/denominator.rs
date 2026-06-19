pub struct Denominator<const SIZE: usize> {
    pub coefficient: [f64; SIZE],
}

impl<const SIZE: usize> Denominator<SIZE> {
    pub fn new(coefficient: [f64; SIZE]) -> Self {
        Denominator { coefficient }
    }

    pub fn get_coefficient(&self, index: usize) -> f64 {
        if index < SIZE {
            self.coefficient[index]
        } else {
            panic!("Index out of bounds");
        }
    }

    pub fn length(&self) -> usize {
        self.coefficient.len()
    }
}
