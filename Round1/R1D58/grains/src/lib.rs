pub fn square(s: u32) -> u64 {
    if s == 0 || s > 64 {
        panic!("Square must be between 1 and 64");
    } else {
        2u64.pow(s - 1)
    }
}

pub fn total() -> u64 {
    // (1..=64).fold(0, |sum, n| sum + square(n))
    (1..=64).map(square).sum()
 }
