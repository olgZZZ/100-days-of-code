// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
// #![allow(unused)]

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    let result: (i16, i16) = (dividend / divisor, dividend % divisor);
    result
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    iter.step_by(2)
}

pub struct Position(pub i16, pub i16);

impl Position {
    pub fn manhattan(&self) -> i16 {
        let result: i16 = self.0.abs() + self.1.abs();
        result
    }
}
