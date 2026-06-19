use crate::vec_deque::SampleBuffer;
use crate::{denominator::Denominator, numerator::Numerator};

pub struct DirectCanonical<const N: usize, const D: usize> {
    pub numerator: Numerator<N>,
    pub denominator: Denominator<D>,
    pub buffer: SampleBuffer,
}

impl<const N: usize, const D: usize> DirectCanonical<N, D> {
    pub fn process(&mut self, x: f64) -> f64 {
        let mut yb_coff = 0.0;

        for (idx, b) in self.denominator.coefficient.iter().skip(1).enumerate() {
            yb_coff += -b * self.buffer.get(idx);
        }

        let b0 = self.denominator.coefficient[0];
        let yb = (yb_coff + x) / b0;

        let mut ya_coff = 0.0;
        for (idx, a) in self.numerator.coefficient.iter().skip(1).enumerate() {
            ya_coff += a * self.buffer.get(idx);
        }

        let a0 = self.numerator.coefficient[0];
        self.buffer.push(yb);

        (yb * a0) + ya_coff
    }
}
