use std::fs;

use ahash::AHashSet;

const WORD_LIST: &str = "fruits.txt";

pub fn read_words() -> crate::Result<Vec<String>> {
    let word_text = fs::read_to_string(WORD_LIST)?;
    let words = word_text
        .lines()
        // filter out empty lines
        .filter(|s| !s.is_empty())
        // collect into hashset to avoid duplications
        .collect::<AHashSet<_>>();
    Ok(words.into_iter().map(str::to_owned).collect())
}
