pub fn is_armstrong_number(num: u32) -> bool {
    let num_string = num.to_string();
    let amount = num_string.len() as u32;

    num_string
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(amount))
        .sum::<u32>()
        == num
}
