pub fn reverse(input: &str) -> String {
    let reverse_string:String = input.chars().rev().collect::<String>();
    println!("Write a function to reverse {}", input);
    reverse_string
}
