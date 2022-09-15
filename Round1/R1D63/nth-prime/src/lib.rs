pub fn nth(n: u32) -> u32 {
    (2..).filter(|&x| is_prime(x)).nth(n as usize).unwrap()
}

fn is_prime(number: u32) -> bool {
    !(2..=(number as f32).sqrt() as u32).any(|x| number % x == 0)
}
