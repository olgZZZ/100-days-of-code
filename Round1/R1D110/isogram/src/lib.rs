pub fn check(candidate: &str) -> bool {
    let mut vec: Vec<char> = candidate
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect();

    vec.sort();
    vec.windows(2).all(|w| w[0] != w[1])
}
