use std::collections::HashMap;

pub fn word_count(words: &str) -> HashMap<String, u32> {
    words
        .split(|c| !char::is_alphanumeric(c) && c != '\'' )
        .filter(|s| !s.is_empty())
        .map(|w| w.trim_matches('\''))
        .fold(HashMap::new(), |mut map, s| {
            *map.entry(s.to_lowercase()).or_insert(0) += 1;
            map
        })
}

