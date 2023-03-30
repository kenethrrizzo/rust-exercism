use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let input = UnicodeSegmentation::graphemes(input, true).collect::<Vec<&str>>();
    let mut reversed_word: String = "".to_string();
    for i in (0..input.len()).rev() {
        reversed_word.push_str(input[i]);
    }

    reversed_word
}
