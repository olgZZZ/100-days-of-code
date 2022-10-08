use rand::{thread_rng, Rng};

pub fn encode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() || key.chars().any(|c| !c.is_ascii_lowercase()) {
        return None;
    }

    let result = s
        .chars()
        .zip(key.chars().map(|c| (c as u8 - b'a')).cycle())
        .map(|(c, k)| {
            if c.is_ascii_alphabetic() {
                shift(c, k)
            } else {
                c
            }
        })
        .collect();

    Some(result)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() || key.chars().any(|c| !c.is_ascii_lowercase()) {
        return None;
    }

    let result = s
        .chars()
        .zip(
            key.chars()
                .map(|c| (26 - (c as u8 - b'a')).rem_euclid(26))
                .cycle(),
        )
        .map(|(c, k)| {
            if c.is_ascii_alphabetic() {
                shift(c, k)
            } else {
                c
            }
        })
        .collect();

    Some(result)
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = thread_rng();

    let key: String = (0..100)
        .map(|_| rng.gen_range(b'a'..b'z') as char)
        .collect();

    let result = encode(&key, s).unwrap();

    (key, result)
}

fn shift(c: char, key: u8) -> char {
    let offset = if c.is_ascii_uppercase() { b'A' } else { b'a' } as i8;

    let codepoint = ((c as i8 - offset) + key as i8).rem_euclid(26);

    ((codepoint + offset) as u8) as char
}
