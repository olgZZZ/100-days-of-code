use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    mod_exp(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    mod_exp(b_pub, a, p)
}

pub fn mod_exp(num: u64, power: u64, modulus: u64) -> u64 {
    let (mut power, mut num, modulus, mut result) =
        (power, num as u128, modulus as u128, 1 as u128);

    while power > 0 {
        if power % 2 == 1 {
            result = (result * num) % modulus;
        }
        num = (num * num) % modulus;
        power = power / 2
    }

    result as u64
}
