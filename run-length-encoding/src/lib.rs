pub fn encode(source: &str) -> String {

    let mut return_string:String = String::new();

    if source.len() == 0 {
        println!("Empty string !");
        return_string
    } else {
        let mut iter = source.chars();
        let mut pre_char: char = '0';
        let mut count: u64 = 0;

        loop {
            if let Some(iterm) = iter.next() {
                if pre_char != iterm {
                    if count != 0 {
                        if count > 1 {
                            return_string.push_str(&count.to_string());
                        }
                        return_string.push(pre_char);
                    }
                    count = 1;
                    pre_char = iterm;
                } else {
                    count += 1;
                }
            } else {
                    if count != 0 {
                        if count > 1 {
                            return_string.push_str(&count.to_string());
                        }
                        return_string.push(pre_char);
                    }
                    break;
            }
        }


        return_string
    }

}

pub fn decode(source: &str) -> String {
    let mut return_string: String = String::new();
    println!("Decode source is {} ." , source);

    loop {
        let mut iter = source.chars();
        let mut temp_string = String::new();
        let count:u64;
        
        if let Some(iterm) = iter.next() {
            if iterm.is_ascii_alphanumeric() {
                temp_string.push(iterm);
            } else {
                count = temp_string.parse().unwrap_or(0);
                for _ in 0..count {
                    return_string.push(iterm);
                }
            }
        } else {
                break;
        }
    }

    return_string
}
