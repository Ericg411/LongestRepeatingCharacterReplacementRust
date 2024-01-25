use std::collections::HashMap;
use std::cmp::max;


pub fn character_replacement(s: String, k: i32) -> i32 {
    let mut left: usize = 0;
    let mut freq_hash: HashMap<char, i32> = HashMap::new();
    let mut most_letters: i32 = 0;
    let mut max_string: i32 = 0;

    for (right, current_char) in s.chars().enumerate() {
        *freq_hash.entry(current_char).or_insert(0) += 1;
        most_letters = max(freq_hash[&current_char], most_letters);
        let letters_to_replace: i32 = (right - left + 1) as i32 - most_letters;
        if letters_to_replace > k  {
            let left_char: char = s.chars().nth(left).unwrap();
            if let Some(x) = freq_hash.get_mut(&left_char) {
                *x -= 1;
            }
            left += 1;
        }

        max_string = max((right - left + 1) as i32, max_string);
    }

    max_string
}

fn main() {
    let result = character_replacement(String::from("ABAB"), 2);
    if result != 4 {
        panic!("Test Failed, expected 4")
    }

    let result = character_replacement(String::from("AABABBA"), 1);
    if result != 4 {
        panic!("Test Failed, expected 4");
    }

    println!("All tests passed!")
}
