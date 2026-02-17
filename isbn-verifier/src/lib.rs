/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    if isbn.len() > 0 {
        println!("isbn string length is {} .", isbn.len());
    } else {
        return false;
    }

    let temp_string = isbn.replace("-", "");

    if temp_string.len() == 10 {
        println!("temp_string string length is {} .", temp_string.len());
    } else {
        return false;
    }
    println!(" =====>>> temp_string is {} .", temp_string);

    let mut index = temp_string.len() + 1;
    let valid_resut: u32 = temp_string
        .chars()
        .map(|x| {
            index -= 1;
            let y: u32;
            if x == 'X' && index == 1 {
                y = 10;
            } else if x.is_ascii_digit() {
                y = x.to_digit(10).unwrap();
            } else {
                y = 255;
            }
            println!("{} * {} = [{}] ", y, index, y * index as u32);
            y * index as u32
        })
        .sum();

    if valid_resut.rem_euclid(11) == 0 {
        return true;
    } else {
        return false;
    }
}
