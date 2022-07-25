use std::collections::HashSet;

pub fn is_pangram(sentence: &str) -> bool {
    let sentence_set: HashSet<char> = sentence.to_lowercase().chars().collect();
    let alphabet: HashSet<char> = (0..26).map(|x| (x + 'a' as u8) as char).collect();

    alphabet.difference(&sentence_set).count() == 0
}