// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;
use std::thread::sleep;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {

    let mut magazine_words: HashMap<&str, i32> = HashMap::new();
    for word in magazine.iter() {
        // println!("Magazine word {}" , word);

        match magazine_words.get(word) {
            Some(word_count) => {
                magazine_words.insert(word, word_count + 1);
            },
            None => {
                magazine_words.insert(word, 1);
            }
        }
    }

    let mut note_words: HashMap<&str, i32> = HashMap::new();
    for word in note.iter() {
        // println!("Note word {}" , word);

        match note_words.get(word) {
            Some(word_count) => {
                note_words.insert(word, word_count + 1);
            },
            None => {
                note_words.insert(word, 1);
            }
        }
    }

    for word in note.iter() {
        match note_words.get(word) {
            Some(note_word_count) => {
                match magazine_words.get(word) {
                    Some(magazine_word_count) => {
                        if note_word_count > magazine_word_count {
                            return false;
                        }
                    },
                    None => {
                        return false;
                    }
                }

            },
            None => {
                return false;
            }
        }

    }

    true
}
