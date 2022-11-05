#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }

    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    let mut num: u32 = 0;

    for digit in number.iter() {
        if *digit >= from_base {
            return Err(Error::InvalidDigit(*digit));
        }
        num = num * from_base + *digit;
    }

    let mut ret: Vec<u32> = Vec::new();

    while num != 0 {
        ret.push(num % to_base);
        num = num / to_base;
    }

    if ret.is_empty() {
        ret.push(0);
    }

    ret.reverse();

    Ok(ret)
}
