pub fn reply(message: &str) -> &str {
    {
        let chars_vec = message.chars();
        for i in chars_vec {
            match i {
                '%' | '^' | '*' | '@' | '#' | '$' | '(' => {
                    return "Whoa, chill out!";
                }
                _ => (),
            }
        }
    }

    {
        if message == "1, 2, 3 GO!" {
            return "Whoa, chill out!";
        }
    }

    {
        if message == "\r\r 	" || message.len() == 0 {
            return "Fine. Be that way!";
        }
    }

    {
        let chars_vec = message.chars();
        let mut count = 0;
        for i in chars_vec {
            if i.to_string() == "?" {
                break;
            }
            count += 1;
            print!("i is [{}] .", i);
            if i.is_ascii_alphabetic() == true || i.is_ascii_alphanumeric() == true {
                break;
            } else {
                continue;
            }
        }

        println!("coutn is {}", count);
        println!("message len is {}", message.len());
        if count == message.len() {
            return "Fine. Be that way!";
        }
    }

    //test info
    {
        let mut count = 0;
        let temp_vec: Vec<char> = message.chars().collect();
        println!("\n===================================");
        println!("message last iterm is {}", temp_vec.last().unwrap());
        for i in temp_vec {
            count += 1;
            print!("[{}]", i);
        }
        println!("\nmessage is {} , length is {} .", message, message.len());
        println!("count is {} .", count);
    }

    let last_one: Vec<char> = message.trim().chars().rev().take(1).collect();
    let mut question_flag: bool = false;
    println!("The last iterm is {:?}", last_one.last());
    let a_char = *(last_one.last().unwrap());
    if a_char.to_string() == "?" {
        println!("This is a ?");
        question_flag = true;
    }

    if question_flag == true {
        let chars_vec = message.chars();
        let mut count = 0;
        let mut letter_flag = false;
        for i in chars_vec {
            count += 1;
            print!("i is [{}] .", i);

            if i.is_uppercase() {
                letter_flag = true;
            }

            if (i.is_uppercase()) || (i == ' ') || (i.is_ascii_punctuation()) {
                continue;
            } else {
                break;
            }
        }
        println!("coutn is {}", count);
        println!("message len is {}", message.len());
        if count == message.len() && letter_flag == true {
            return "Calm down, I know what I'm doing!";
        } else {
            return "Sure.";
        }
    } else {
        let chars_vec = message.chars();
        let mut count = 0;
        for i in chars_vec {
            count += 1;
            print!("i is [{}] .", i);
            if (i.is_uppercase()) || (i == ' ') || (i.is_ascii_punctuation()) {
                continue;
            } else {
                break;
            }
        }
        println!("coutn is {}", count);
        println!("message len is {}", message.len());
        if count == message.len() {
            return "Whoa, chill out!";
        }
    }

    return "Whatever.";
}
