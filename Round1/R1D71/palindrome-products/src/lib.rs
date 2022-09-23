#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.

    pub fn new(value: u64) -> Option<Palindrome> {
        if reverse_value(value) == value {
            Some(Palindrome(value))
        } else {
            None
        }
    }

    /// Get the value of this palindrome.

    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut tuple: Option<(u64, u64)> = None;

    for i in min..=max {
        for j in min..=max {
            let k = i * j;

            if k.rem_euclid(10) == 0 {
                continue;
            }

            if k == reverse_value(k) {
                tuple = match tuple {
                    Some((x, y)) => Some((x, y.max(k))),

                    None => Some((k, k)),
                }
            }
        }
    }

    if let Some((x, y)) = tuple {
        Some((Palindrome(x), Palindrome(y)))
    } else {
        None
    }
}

fn reverse_value(value: u64) -> u64 {
    let mut reverse = 0;
    
    let mut copy = value;

    while copy > 0 {
        reverse *= 10;

        reverse += copy % 10;

        copy /= 10;
    }

    reverse
}
