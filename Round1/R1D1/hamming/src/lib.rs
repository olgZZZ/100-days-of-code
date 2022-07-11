/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
use std::iter::zip;

pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        None
    } else {
        let distance = zip(s1.chars(), s2.chars())
            .filter(|(s1, s2)| s1 != s2)
            .count();
        Some(distance)
    }
}
