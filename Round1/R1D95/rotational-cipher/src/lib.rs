pub fn rotate(input: &str, key: i8) -> String {
    let rotate_char =
        |c: u8, base: u8| (((c - base) as i8 + key)
        .rem_euclid(26) as u8 + base) as char;

    input
        .chars()
        .map(|c| {
            if c.is_uppercase() {
                rotate_char(c as u8, b'A')
            } else if c.is_lowercase() {
                rotate_char(c as u8, b'a')
            } else {
                c
            }
        })
        .collect::<String>()
}
