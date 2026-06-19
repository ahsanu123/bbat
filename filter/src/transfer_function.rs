use crate::{
    denominator::Denominator, direct_canonical::DirectCanonical, numerator::Numerator,
    vec_deque::SampleBuffer,
};

pub struct TransferFunction<const N: usize, const D: usize> {
    pub numerator: Numerator<N>,
    pub denominator: Denominator<D>,
}

impl<const N: usize, const D: usize> TransferFunction<N, D> {
    pub fn new(numerator: Numerator<N>, denominator: Denominator<D>) -> Self {
        if N > D {
            panic!(
                "The order of the numerator cannot be greater than the order of the denominator"
            );
        }

        TransferFunction {
            numerator,
            denominator,
        }
    }

    pub fn to_beauty_string(&self) -> String {
        let mut s = String::new();

        for (idx, n) in self.numerator.coefficient.iter().enumerate() {
            if idx == 0 {
                s.push_str(&format!("{} ", n));
            } else {
                s.push_str(&format!(" + {} z^-{}", n, idx));
            }
        }
        s.push('\n');

        let len = s.len();
        for _ in 1..len {
            s.push('-');
        }

        s.push('\n');

        for (idx, d) in self.denominator.coefficient.iter().enumerate() {
            if idx == 0 {
                s.push_str(&format!("{} ", d));
            } else {
                s.push_str(&format!(" + {} z^-{}", d, idx));
            }
        }

        s
    }

    pub fn build_direct_canonical(self) -> DirectCanonical<N, D> {
        DirectCanonical {
            numerator: self.numerator,
            denominator: self.denominator,
            buffer: SampleBuffer::new(D),
        }
    }
}
