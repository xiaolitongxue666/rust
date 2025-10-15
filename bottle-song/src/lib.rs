/// Convert number to word representation
fn number_to_word(n: u32) -> &'static str {
    match n {
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        10 => "Ten",
        _ => "Out of range",
    }
}

/// Generate a single verse
fn verse(n: u32) -> String {
    let current_word = number_to_word(n);
    let next_n = n - 1;

    let next_word = if next_n == 0 {
        "no".to_string()
    } else if next_n == 1 {
        "one".to_string()
    } else {
        number_to_word(next_n).to_lowercase()
    };

    let bottle_word = if next_n == 1 { "bottle" } else { "bottles" };
    let current_bottle_word = if n == 1 { "bottle" } else { "bottles" };

    format!(
        "{} green {} hanging on the wall,\n\
         {} green {} hanging on the wall,\n\
         And if one green bottle should accidentally fall,\n\
         There'll be {} green {} hanging on the wall.",
        current_word,
        current_bottle_word,
        current_word,
        current_bottle_word,
        next_word,
        bottle_word
    )
}

/// Recite the bottle song from start_bottles down by take_down verses
pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut verses = Vec::new();

    for i in 0..take_down {
        let current = start_bottles - i;
        if current > 0 {
            verses.push(verse(current));
        }
    }

    verses.join("\n\n")
}
