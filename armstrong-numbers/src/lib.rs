pub fn is_armstrong_number(num: u32) -> bool {
    let mut consult_numer_vec: Vec<u32> = Vec::new();
    let mut sum = 0;
    let mut divisor: u32 = 1;
    let mut reminder;

    if num == 0 {
        return true;
    }

    loop {
        reminder = num / divisor % 10;
        if reminder != 0 {
            consult_numer_vec.push(reminder);
            divisor *= 10;
        } else {
            break;
        }
    }

    let numbers = consult_numer_vec.len();
    if numbers == 0 {
        return false;
    }

    for i in consult_numer_vec {
        println!(" vec iterm valus is {}", i);
        sum += i.pow(numbers as u32);
    }

    return if sum == num { true } else { false };
}
