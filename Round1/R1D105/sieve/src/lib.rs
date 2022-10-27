pub fn primes_up_to(upper_bound: u32) -> Vec<u32> {
    let mut range: Vec<u32> = (2..=upper_bound).collect();
    let mut primes: Vec<u32> = Vec::new();
    while !range.is_empty() {
        let first = range[0];
        primes.push(first);
        range = range
            .into_iter()
            .skip(1)
            .filter(|num| num % first != 0)
            .collect();
    }
    primes
}