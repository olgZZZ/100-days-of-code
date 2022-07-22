use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut result = HashSet::new();
    for a in 1..sum {
        for b in 1..sum - a {
            let c = sum - a - b;
            if a * a + b * b == c * c {
                if a < b && b < c {
                    result.insert([a, b, c]);
                }
            }
        }
    }
    result
}