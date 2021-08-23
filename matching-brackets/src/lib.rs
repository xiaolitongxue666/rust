pub fn brackets_are_balanced(string: &str) -> bool {

    let mut itor_for_chars = string.chars();
    let mut vec = Vec::new();
    while let Some(c) = itor_for_chars.next() {
        println!(" Char is : \"{}\" " , c);
        
        match c {
            '{' => {
                println!(" Find a {{");
                vec.push(c)
            },

            '}' => {
                println!(" Find a }}");

                if vec.is_empty() {
                    print!("Too many }}");
                    return false;
                }

                if let Some(pop_char) = vec.pop() {
                    if pop_char == '{' {
                        println!("Match a pair of {{}}");
                    } else {
                        println!("Don't match a pair of {{}}");
                        return false
                    }
                }
            }

            '[' => {
                println!(" Find a [");
                vec.push(c);
            },


            ']' => {
                println!(" Find a ]");

                if vec.is_empty() {
                    print!("Too many ]");
                    return false;
                }

                if let Some(pop_char) = vec.pop() {
                    if pop_char == '[' {
                        println!("Match a pair of []");
                    } else {
                        println!("Don't match a pair of []");
                        return false
                    }
                }
            }

            '(' => {
                println!(" Find a (");
                vec.push(c);
            },


            ')' => {
                println!(" Find a )");

                if vec.is_empty() {
                    print!("Too many )");
                    return false;
                }

                if let Some(pop_char) = vec.pop() {
                    if pop_char == '(' {
                        println!("Match a pair of ()");
                    } else {
                        println!("Don't match a pair of ()");
                        return false
                    }
                }
            }

            _ => (),
        }
    }

    if vec.is_empty() {
        true
    } else {
        false
    }
}
