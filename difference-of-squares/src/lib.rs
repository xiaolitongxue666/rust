pub fn square_of_sum(n: u32) -> u32 {
    let mut restult: u32 = 0;
    for i in 1..n + 1 {
        restult += i;
    }
    println!("The sum is {} .", restult);
    restult *= restult;
    println!("The square of sum is {} .", restult);
    restult
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut restult: u32 = 0;
    for i in 1..n + 1 {
        restult += i * i;
    }
    println!("The square of sum is {} .", restult);
    restult
}

pub fn difference(n: u32) -> u32 {
    let result: u32;
    result = square_of_sum(n) - sum_of_squares(n);
    result
}
