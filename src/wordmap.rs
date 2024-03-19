//! Jumble Helper for Mom (FEB 2024)

use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;

/// Stores an entire dictionary in `WordMap`s sorted by word length.
struct WordMaps {
    maps: Vec<WordMap>,
}

impl WordMaps {
    /// Creates a new `WordMaps` instance.
    fn new(max_word_len: usize) -> Self {
        let mut maps = Vec::new();
        for _ in 0..(max_word_len + 1) {
            let wm = WordMap::new();
            maps.push(wm);
        }
        Self { maps }
    }
}

/// Stores all words in {ordered_word, [actual_words]} format.
///
/// Each ordered word represents the corresponding actual word(s) whose characters
/// have been sorted in alphabetical order.
pub struct WordMap {
    inner: HashMap<String, Vec<String>>,
}

impl WordMap {
    /// Creates a new `WordMap` instance.
    pub fn new() -> Self {
        Self {
            inner: Default::default(),
        }
    }
    /// Adds a sorted key and its unsorted (actual) value to the word map.
    ///
    /// If the word, when sorted, is *not* in the map, a new entry is created. If
    /// it *is* in the map, the unsorted (actual) word is added to the existing entry.
    pub fn insert(&mut self, sorted: String, unsorted: String) {
        // println!("[Wordmap.insert] inserting {}, {}", unsorted, sorted);
        self.inner.entry(sorted).or_default().push(unsorted);
    }
    /// Returns the words, if any, that match the given unsorted query.
    pub fn find_match(&self, q: &str, minlen: usize, maxlen: usize) -> Option<&Vec<String>> {
        if q.len() < minlen || q.len() > maxlen {
            return None;
        }
        let sorted_q = q.chars().sorted().collect::<String>();
        self.inner.get(&sorted_q)
    }
    /// Iterates over inner HashMap.
    pub fn iter(&self) -> std::collections::hash_map::Iter<String, Vec<String>> {
        self.inner.iter()
    }
    /// Returns the number of words in the map.
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

/// Loads words from dictionary text file.
pub fn load_words_to_string(filename: &str) -> io::Result<String> {
    let mut f = File::open(filename)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    println!("[load_words_to_string] First 10 words:");
    for word in buffer.lines().take(10) {
        println!("{}", word);
    }

    Ok(buffer)
}

/// Takes word list as a `String` and converts them into a list of words whose
/// letters are arranged in alphabetical order.
fn sorted_words(words: String) -> Vec<String> {
    println!("[sorted_words] First 10 words:");
    for word in words.lines().take(10) {
        println!("{}", word);
    }
    // let ordered_words = words.lines().map(|&word| word.sort() ).collect::<Vec>();

    let mut ordered_words = Vec::with_capacity(100000);

    for word in words.lines() {
        let sorted_word = word.chars().sorted().collect::<String>();
        ordered_words.push(sorted_word);
    }

    ordered_words
}

/// Converts word list to a map of {ordered_word, [actual word, ...]} pairs.
///
/// Ordered words are the actual word(s) whose characters have been arranged in
/// alphabetical order.
///
/// Example:
///
/// ```python
/// {
///    "aet": ["eat", "tea"],
///    "art": ["art", "rat", "tar"],
///    "eprsuu": ["pursue"],
///    "rsttu": ["trust"],
/// }
/// ```
pub fn make_word_map(words: &str) -> WordMap {
    println!("[words_to_word_map]");

    let mut word_map = WordMap::new();

    for word in words.lines() {
        let sorted_word = word.chars().sorted().collect::<String>();
        word_map.insert(sorted_word, word.to_string());
    }

    word_map
}

fn make_word_map_string(words: String) -> WordMap {
    println!("[words_to_word_map]");

    let mut word_map = WordMap::new();

    for word in words.lines() {
        let sorted_word = word.chars().sorted().collect::<String>();
        word_map.insert(sorted_word, word.to_string());
    }

    word_map
}

fn main() {
    println!("=== Jumble Helper ===");

    let words = load_words_to_string("dictionary/ENGLISH_US.txt").expect("Valid text file");
    // let ordered_words = sorted_words(words);

    // println!("Second word: {}", ordered_words[1]);

    // let mut word_map = make_word_map(words);
    // for pair in word_map.iter().take(25) {
    //     println!("{pair:?}");
    // }
    // let num_words = word_map.inner.len();
    // println!("number of words: {num_words}");
}
