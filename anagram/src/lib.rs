use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let sorted_word: String = sort_and_up_word(word);

    let mut result: HashSet<&'a str> = HashSet::new();
    for anagram in possible_anagrams.iter() {
        if eq_ignore_case(word, anagram) {
            continue;
        }
        if sorted_word == sort_and_up_word(anagram) {
            result.insert(anagram);
        }
    }
    return result;
}

fn sort_and_up_word(word: &str) -> String {
    let mut chars: Vec<char> = word.to_uppercase().chars().collect();
    chars.sort_unstable();
    return chars.into_iter().collect();
}

fn eq_ignore_case(first_string: &str, second_string: &str) -> bool {
    return first_string.to_lowercase() == second_string.to_lowercase();
}
