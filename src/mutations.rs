use rand::Rng;

pub fn substitute_char(index: usize, text: &str, substitution: char) -> String {
    let mut text_before: String = text[0 .. index].to_owned();
    let text_after: &str = &text[(index + 1) .. (text.len())];

    text_before.push(substitution);
    text_before.push_str(text_after);

    return text_before;
}

pub fn choose_substitution(original: char) -> char {
    let mut rng = rand::thread_rng();

    const DIFFERENCES: [i32; 2] = [-1, 1];

    let difference_index = rng.gen_range(0 .. 2);

    let original_ascii: i32 = original as i32;

    let mut substitution_ascii: i32 = original_ascii + DIFFERENCES[difference_index];

    if substitution_ascii >= 128 {
        substitution_ascii = 0;
    }
    else if substitution_ascii <= 0 {
        substitution_ascii = 127;
    }

    let substitution: char = substitution_ascii as u8 as char;
    return substitution;
}