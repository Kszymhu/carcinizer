use std::cmp::{min, max};

const ASCII_TABLE_LENGTH: i32 = 128;

fn get_char_distance(pattern: char, target: char) -> i32 {
    let pattern_ascii: i32 = pattern as i32; // char as i32 returns the ASCII code
    let target_ascii: i32 = target as i32;

    let regular: i32 = (pattern_ascii - target_ascii).abs();
    let wraparound: i32 = ASCII_TABLE_LENGTH - regular;

    return min(regular, wraparound);
}

fn get_word_distance(pattern: &str, target: &str) -> i32 {
    let mut word_distance: i32 = 0;

    if pattern.len() < target.len() {
        panic!("Target cannot be longer than the pattern.");
    }

    let pattern_chars: Vec<char> = pattern.chars().collect(); // Getting a vector of chars
    let target_chars: Vec<char> = target.chars().collect();

    for i in 0 .. pattern.len() {

        if i >= target.len() { // If target is shorter than the pattern, each empty space is "priced" at 128, ensuring that it's the "worst" character
            let remaining_length: i32 = (pattern.len() - target.len()) as i32;
            word_distance += ASCII_TABLE_LENGTH * remaining_length;
            break;
        }

        let char_distance: i32 = get_char_distance(
            pattern_chars[i],
            target_chars[i]
        );

        word_distance += char_distance;
    }

    return word_distance;
}

pub fn get_string_distance(pattern: &str, target: &str) -> i32 {
    let word_count: usize = max( // Ensures that if target is shorter than the pattern, there will still be a word to check
        target.len() as i32 - pattern.len() as i32, 
        1
    ) as usize;

    if target.len() <= pattern.len() { // If target is shorter than the pattern, there's no need to slice anything
        return get_word_distance(pattern, target);
    }

    let mut min_distance: i32 = i32::MAX;

    for start_index in 0 .. word_count {
        let end_index: usize = start_index + pattern.len();
        let target_word: &str = &target[start_index .. end_index];
        let word_distance: i32 = get_word_distance(pattern, target_word);
        
        if word_distance < min_distance {
            min_distance = word_distance;
        }
    }

    let remaining_length: i32 = (target.len() - pattern.len()) as i32; // Treats all "trailing" (non-best-crab) characters same as get_word_distance treats empty spaces
    return min_distance + remaining_length * ASCII_TABLE_LENGTH;
}