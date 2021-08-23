pub fn raindrops(n: u32) -> String {

    let mut result_string = String::from("");

    if n%3 == 0 || n%5 == 0 || n%7 == 0 {

        if n%3 == 0 {
            result_string.push_str("Pling");
            println!("Add a Pling to string");
        }
    
        if n%5 == 0 {
            result_string.push_str("Plang");
            println!("Add a Plang to string");
        }
    
        if n%7 == 0 {
            result_string.push_str("Plong");
            println!("Add a Plong to string");
        }
    } else {
        result_string = n.to_string();
    }

    result_string
}
