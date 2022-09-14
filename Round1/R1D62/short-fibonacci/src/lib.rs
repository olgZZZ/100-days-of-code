/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    Vec::new()
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    vec![0; count]
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    let mut buffer = create_buffer(5);

    for i in 0..5 {
        buffer[i] = fib(i as u8);
    }

    buffer
}


pub fn fib(n: u8) -> u8 {
    if n == 0 || n == 1 {
        return 1;
    }

    let mut sum: u8 = 0;
    let mut last: u8 = 0;
    let mut curr: u8 = 1;

    for _i in 0..n {
        sum = last + curr;
        last = curr;
        curr = sum;
    }

    sum
}

