/*
 * Design Path
 * First create a function can conculate next prime number
 * Second finish factors funciton
 *
 * */
pub fn is_prime(input_number: u64) -> bool {
    if input_number < 2 {
        return false;
    }
    let limit = (input_number as f64).sqrt() as u64 + 1;
    for i in 2..=limit {
        if input_number % i == 0 {
            return false;
        }
    }
    true
}

pub fn next_prime(now_prime: u64) -> u64 {
    let mut temmp_test_number = now_prime;
    loop {
        temmp_test_number += 1;
        if is_prime(temmp_test_number) {
            return temmp_test_number;
        }
    }
}

pub fn factors(n: u64) -> Vec<u64> {
    let mut return_vec: Vec<u64> = Vec::new();
    if n < 2 {
        return return_vec;
    } else {
        let mut divisor = 2;
        let mut devidend = n;
        let mut left_number;
        loop {
            if devidend % divisor == 0 {
                return_vec.push(divisor);
                left_number = devidend / divisor;
                if left_number == 1 {
                    return return_vec;
                } else {
                    devidend = left_number;
                }
            } else {
                divisor += if divisor == 2 { 1 } else { 2 };
            }
        }
    }
}
