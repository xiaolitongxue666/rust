/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    vec![]
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    // unimplemented!("create a zeroized buffer of {} bytes", count)
    let vec = vec![0; count];
    vec
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    let mut fib_vec = create_buffer(5);

    for index in 0..fib_vec.len() {
        if index < 2 {
            fib_vec[index] = 1;
        } else {
            fib_vec[index] = fib_vec[index - 1] + fib_vec[index -2];
        }
    }

    fib_vec
}
