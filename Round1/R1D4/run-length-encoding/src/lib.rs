use std::iter;

pub fn encode(source: &str) -> String {
    let mut result = String::new();
    let mut chars = source.chars().peekable();
    let mut count = 0;

    while let Some(c) = chars.next() {
        count += 1;

        if chars.peek() != Some(&c) {
            if count > 1 {
                result.push_str(&count.to_string());
            }
            result.push_str(&c.to_string());
            count = 0;
        }
    }
    result
}

pub fn decode(source: &str) -> String {
    source
        .chars()
        .filter(|c| !c.is_numeric())
        .zip(
            source
                .split(|c: char| !c.is_numeric())
                .map(|n| n.parse::<usize>().unwrap_or(1)),
        )
        .flat_map(|(c, n)| iter::repeat(c).take(n))
        .collect()
}

