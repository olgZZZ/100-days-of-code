pub fn encrypt(input: &str) -> String {
    let mut normalized: Vec<char> = input
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();

    let (r, c) = (0..)
        .flat_map(|i| [(i, i), (i, i + 1)])
        .find(|(c, r)| c * r >= normalized.len())
        .unwrap();

    if c == 0 {
        return "".to_string();
    }

    normalized.append(&mut vec![' '; r * c - normalized.len()]);

    let rows: Vec<_> = normalized.chunks(c).collect();

    (0..c)
        .map(|j| rows.iter().map(|r| r[j]).collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}
