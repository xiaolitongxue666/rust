pub fn verse(n: u32) -> String {
    let mut temp_string: String = n.to_string();
    let mut return_string: String = "".to_string();

    if n == 0 {
        return_string = "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string();
    } else if n == 1 {
        return_string = "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string();
    } else {
        return_string.push_str(&temp_string);
        return_string.push_str(" bottles of beer on the wall, ");
        return_string.push_str(&temp_string);
        return_string.push_str(" bottles of beer.\nTake one down and pass it around, ");
        let m = n - 1;
        temp_string = m.to_string();
        return_string.push_str(&temp_string);
        if m == 1 {
            return_string.push_str(" bottle of beer on the wall.\n")
        } else {
            return_string.push_str(" bottles of beer on the wall.\n")
        }
    }

    return_string
}

pub fn sing(start: u32, end: u32) -> String {
    let mut return_string: String = "".to_string();
    let mut i = start;
    while i >= end {
        return_string.push_str(&verse(i));
        if i != end {
            return_string.push_str("\n");
        }
        if i != 0 {
            i -= 1;
        }
    }
    return_string
}
