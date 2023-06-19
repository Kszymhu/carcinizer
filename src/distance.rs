use std::cmp::min;

const ASCII_TABLE_LENGTH: i32 = 128;

/// Calculates the difference between ASCII codes of [a] and [b], treating the ASCII table as a ring.
fn get_char_distance(a: char, b: char) -> i32 {
    let a_ascii: i32 = a as i32; // char as i32 returns the ASCII code
    let b_ascii: i32 = b as i32;

    let regular: i32 = (b_ascii - a_ascii).abs();
    let wraparound: i32 = ASCII_TABLE_LENGTH - regular;

    return min(regular, wraparound);
}

/// Calculates the sum of distances between each letter of [text] and [character].
fn get_word_distance_to_char(text: &str, character: char) -> i32 {
    let mut distance: i32 = 0;
    let chars: Vec<char> = text.chars().collect();

    for char in chars {
        distance += get_char_distance(char, character);
    }

    return distance;
}

/// Calculates the sum of distances between corresponding chars of [a] and [b].
fn get_word_distance(a: &str, b: &str) -> i32 {

    if a.len() != b.len() {
        panic!("Both words have to be of the same length!");
    }

    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();

    let mut word_distance: i32 = 0;

    for i in 0 .. a.len() {
        let a_char: char = a_chars[i];
        let b_char: char = b_chars[i];
        let distance = get_char_distance(a_char, b_char);
        word_distance += distance;
    }

    return word_distance;
}

/// Looks for a substring of [text] which has the smallest distance from [pattern].
/// 
/// Returns a tuple of (best_distance, best_index), where
/// - best_distance - distance from best match
/// - best_index - index in [text] of the first letter of the match
fn get_best_match(pattern: &str, text: &str) -> (i32, usize) {
    if pattern.len() > text.len() {
        panic!("Text cannot be shorter than the pattern!");
    }

    let word_count: usize = text.len() - pattern.len();

    let mut best_distance: i32 = i32::MAX;
    let mut best_index: usize = 0;

    for start_index in 0 .. word_count {
        let end_index: usize = start_index + pattern.len() - 1;
        let word: &str = &text[start_index ..= end_index];

        let distance = get_word_distance(word, pattern);

        if distance < best_distance {
            best_distance = distance;
            best_index = start_index;
        }
    }

    return (best_distance, best_index);
}

/// Finds the best match of [pattern] to [text], and then calculates corresponding distances of [text] to
/// match padded with spaces to the length of [text].
/// 
/// Returns a tuple of (best_distance, best_index), where
/// - best_distance - distance from best match
/// - best_index - index in [text] of the first letter of the match
pub fn get_string_distance(pattern: &str, text: &str) -> (i32, usize) {
    let best_match_result: (i32, usize) = get_best_match(pattern, text);
    let best_match_distance: i32 = best_match_result.0;
    let best_match_start: usize = best_match_result.1;
    let best_match_end: usize = best_match_start + pattern.len();

    let chars_before: &str = &text[0 .. best_match_start];
    let chars_after: &str = &text[best_match_end .. text.len()];

    let chars_before_distance: i32 = get_word_distance_to_char(chars_before, ' ');
    let chars_after_distance: i32 = get_word_distance_to_char(chars_after, ' ');

    let distance: i32 = chars_before_distance + best_match_distance + chars_after_distance;

    return (distance, best_match_start);
}