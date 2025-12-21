// This chapter is dedicated to the ownership, borrowing and slices

// OWNERSHIP
// ================================================================================================

// ----- 1 --------------------------------------
// Write a function `longest_owned(s1: String, s2: String) -> String` that returns the longer of
// two strings. Check that both original strings are moved into the function, and only the returned
// can still be used.
//
// You can implement the function and use it right inside the `string_ownership` function.
#[allow(dead_code)]
pub fn string_ownership() {
    fn longest_owned(s1: String, s2: String) -> String {
        if s1.len() > s2.len() {
            s1
        } else {
            s2
        }
    }

    let str1 = String::from("short");
    let str2 = String::from("long123");

    let result = longest_owned(str1, str2);
    println!("{}", result);
}

// BORROWING
// ================================================================================================

// ----- 2 --------------------------------------
// Write a function `print_length(s: ???)` that takes some string and prints its length without
// taking ownership. First use it with some random (censored) string, and then print this string to
// show that it was not moved and still available.
//
// You can implement the function and use it right inside the `simple_borrowing` function.
#[allow(dead_code)]
pub fn simple_borrowing() {
    fn print_length(s: &str) {
        println!("{}", s.len());
    }

    let my_string = String::from("Hello, Rust!");
    print_length(&my_string); 
    println!("{}", my_string);
}

// ----- 3 --------------------------------------
// Implement a function `append_and_return_length(string: ???, suffix: ???) -> usize` that borrows
// some string, appends a suffix to it, and returns the new length. Then call it multiple times
// to check that the string was borrowed, not moved.
//
// You can implement the function and use it right inside the `hard_borrowing` function.
#[allow(dead_code)]
pub fn hard_borrowing() {
    fn append_and_return_length(s: &mut String, suffix: &str) -> usize {
        s.push_str(suffix);
        s.len()
    }

    let mut my_string = String::from("Hello");
    let length = append_and_return_length(&mut my_string, ", world!");
    println!("string {}", my_string);
    println!("length: {}", length);
}

// SLICES
// ================================================================================================

// ----- 4 --------------------------------------
// Write a function last_word(s: &str) -> &str that returns the last word from a string slice.
// Assume words are separated by spaces.
pub fn last_word(slice: &str) -> &str {
    match slice.split_whitespace().last() {
        Some(word) => word,
        None => "",
    }
}

// ----- 5 --------------------------------------
// Write a function longest_word(sentence: &str) -> &str that returns the longest word in a
// sentence (string slice). If several words have the same maximum length, return the last one.
pub fn longest_word(sentence: &str) -> &str {
    sentence.split_whitespace()
        .max_by_key(|word| word.len())
        .unwrap_or("")
}
