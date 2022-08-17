pub fn translate(input: &str) -> String {
    input
        .split_ascii_whitespace()
        .map(|word| match (&word[0..1], &word[0..2], &word[0..3]) {
            (_, _, "squ" | "thr" | "sch") => format!("{}{}ay", &word[3..], &word[0..3]),
            (_, "yt" | "xr", _) => format!("{}ay", word),
            (_, "qu" | "ch" | "th" | "rh", _) => format!("{}{}ay", &word[2..], &word[0..2]),
            ("a" | "e" | "i" | "o" | "u", _, _) => format!("{}ay", word),
            _ => format!("{}{}ay", &word[1..], &word[0..1]),
        })
        .collect::<Vec<String>>()
        .join(" ")
}