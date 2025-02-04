use range_check::Check;

pub fn square(s: u32) -> u64 {
    let square = 1..65;
    let range_check_result = s.check_range(square);
    match range_check_result {
        Ok(v) => println!("{} is in range 1..64, {:?} ." , s, v),
        Err(e) => {
            println!("Error {:?}", e);
            panic!("Square must be between 1 and 64");
        },
    }
    2u64.pow(s-1)
}

pub fn total() -> u64 {
    let mut total:u64 = 0;
    for i in 1..65 {
        total += square(i);
    }
    total
}
