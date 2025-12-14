// This chapter is dedicated to some collections: vectors, strings and hash maps

use std::collections::{HashMap, HashSet};

// VECTORS
// ================================================================================================

// ----- 1 --------------------------------------
// Write a function `second_largest(vec: &[i32]) -> Option<i32>` that returns the second largest
// element in the array. If the array has fewer than 2 elements, return `None`.

pub fn second_largest(vec: &[i32]) -> Option<i32> {
    let mut unique_numbers = vec.to_vec();
    unique_numbers.sort_unstable();
    unique_numbers.dedup();

    if unique_numbers.len() < 2 {
        None
    } else {
        Some(unique_numbers[unique_numbers.len() - 2])
    }
}

// ----- 2 --------------------------------------
// Write a function `longest_increasing_subsequence(vec: &[i32]) -> Vec<i32>`` that finds the
// longest strictly increasing subsequence (not necessarily contiguous) in the array.
//
// For the simplicity, assume that there is only one longest increasing subsequence.

pub fn longest_increasing_subsequence(init_sequence: &[i32]) -> Vec<i32> {
    let mut subsequences = vec![Vec::new(); init_sequence.len()];
    subsequences[0].push(init_sequence[0]);

    for i in 1..init_sequence.len() {
        for j in 0..i {
            if init_sequence[i] > init_sequence[j] && subsequences[j].len() > subsequences[i].len() {
                subsequences[i] = subsequences[j].clone();
            }
        }
        subsequences[i].push(init_sequence[i]);
    }

    subsequences.into_iter().max_by_key(|seq| seq.len()).unwrap_or(vec![])
}

// STRINGS
// ================================================================================================

// ----- 3 --------------------------------------
// Write a function `reverse_words(sentence: &str) -> String` that reverses the order of words in a
// sentence but does not reverse the characters inside each word.

pub fn reverse_words(sentence: &str) -> String {
    let words: Vec<&str> = sentence.split_whitespace().collect();
    words.into_iter().rev().collect::<Vec<&str>>().join(" ")
}

// ----- 4 --------------------------------------
// Write a function `normalize_and_capitalize(sentence: &str) -> String` that:
// - Trims extra spaces at the beginning and end.
// - Converts multiple spaces between words into a single space.
// - Makes the first letter of every word uppercase, and every other letter lowercase, for example
//   "пРеВеД МеДвЕд -> Превед Медвед"

pub fn normalize_and_capitalize(sentence: &str) -> String {
    sentence
        .split_whitespace()
        .map(|word| {
            let c = word.to_lowercase();
            let first = c.chars().next().unwrap_or(' ').to_uppercase().to_string();
            first + &c[1..]
        })
        .collect::<Vec<String>>()
        .join(" ")
}

// HASH SET
// ================================================================================================

// ----- 5 --------------------------------------
// Write a function `unique_chars(s: &str) -> bool` that returns true if a string has all unique
// characters (ignoring case), and false otherwise.

pub fn unique_chars(s: &str) -> bool {
    let mut chars = HashSet::new();
    s.to_lowercase()
        .chars()
        .all(|c| chars.insert(c))
}

// HASH MAP
// ================================================================================================

// ----- 6 --------------------------------------
// Write a function `top_k_frequent(nums: Vec<i32>, k: usize) -> Vec<i32>` that returns the `k` most
// frequent numbers in the vector. If `k` is greater than the total number of unique elements in the
// vector, return all of them.

pub fn top_k_frequent(nums: Vec<i32>, k: usize) -> Vec<i32> {
    let mut freq_map = HashMap::new();

    for &num in &nums {
        *freq_map.entry(num).or_insert(0) += 1;
    }

    let mut freq_vec: Vec<_> = freq_map.into_iter().collect();
    freq_vec.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by frequency in descending order

    freq_vec.into_iter().take(k).map(|(num, _)| num).collect()
}
