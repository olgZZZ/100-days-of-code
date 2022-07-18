#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if string_digits.len() < span {
        return Err(Error::SpanTooLong);
    } 
 
    if span == 0 {
        return Ok(1)
    }

    let digits: Vec<u32> = string_digits.chars()
        .map(|c|c.to_digit(10)
        .ok_or(Error::InvalidDigit(c)))
        .collect::<Result<Vec<u32>, Error>>()?;

    let result = digits.windows(span)
        .fold(0, |acc, window| {
            let product = window.iter().product();

            if product > acc { product } else { acc }
        });

    Ok(result as u64)
}