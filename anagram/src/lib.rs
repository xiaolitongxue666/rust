use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // unimplemented!(
    //     "For the '{}' word find anagrams among the following words: {:?}",
    //     word,
    //     possible_anagrams
    // );

    let mut anagrams = HashSet::new();

    for iterm in possible_anagrams.iter() {
        // println!("Possible anagrams : {}", iterm);

        if iterm.len() == word.len() && word.ne(*iterm) {
            // println!("Possible anagrams : {:?}", iterm);
            // println!("Word : {:?}

            let temp_word = word.clone().to_lowercase();
            let temp_iterm = iterm.clone().to_lowercase();
            println!("Temp word : {}", temp_word);
            println!("Temp iterm : {}", temp_iterm);

            if temp_word == temp_iterm {
                continue;
            }

            let mut sort_word: Vec<char> = temp_word.chars().collect();
            sort_word.sort_unstable();
            println!("Sort word : {:?}", sort_word);

            let mut sort_iterm: Vec<char> = temp_iterm.chars().collect();
            sort_iterm.sort_unstable();
            println!("Sort iterm : {:?}", sort_iterm);

            let sort_word_string: String = sort_word.into_iter().collect();
            let sort_iterm_string: String = sort_iterm.into_iter().collect();
            println!("Sort word string : {:?}", sort_word_string);
            println!("Sort iterm string : {:?}", sort_iterm_string);

            if sort_word_string.eq(&sort_iterm_string) {
                anagrams.insert(*iterm);
            }
        }
    }

    anagrams
}
