use std::char;

#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    }

    return gcd(b, a % b);
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if gcd(a, 26) != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }

    Ok(plaintext
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace() && c.is_alphanumeric())
        .map(|c| {
            if c.is_digit(10) {
                c
            } else {
                char::from_u32(
                    ((((c as i32) - 97i32) * a as i32 + b as i32) % 26i32 + 97i32) as u32,
                )
                .unwrap_or(' ')
            }
        })
        .enumerate()
        .map(|(i, v)| {
            if i % 5 == 0 && i != 0 {
                " ".to_string() + &v.to_string()
            } else {
                v.to_string()
            }
        })
        .collect::<String>())
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if gcd(a, 26) != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }

    let mut inv = 0;

    for x in 0..=26 {
        if (x * a) % 26 == 1 {
            inv = x;
            break;
        }
    }
    Ok(ciphertext
        .chars()
        .filter(|c| !c.is_whitespace() && c.is_ascii_alphanumeric())
        .map(|c| {
            if c.is_digit(10) {
                c
            } else {
                char::from_u32((((((c as i32 - 97i32) - b) % 26 + 26) * inv) % 26 + 97) as u32)
                    .unwrap_or('a')
            }
        })
        .collect::<String>())
}
