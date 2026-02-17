pub fn nth(n: u32) -> u32 {
    let mut index: u32 = 0;
    let mut value: u32 = 3;
    let mut i_flag = 1;
    println!("Input para n is {} .", n);
    loop {
        println!("Test value {} .", value);
        if n == 0 {
            value = 2;
            break;
        }

        for i in 2..value {
            i_flag = i;
            if value % i == 0 {
                println!("{} is not a prime .", value);
                break;
            }
        }

        if (i_flag + 1) == value {
            println!("{} is a prime .", value);
            index += 1;
        }

        println!("n is {} , Index is {} , value is {} .", n, index, value);
        if index == n {
            break;
        } else {
            value += 1;
        }
    }
    value
}
