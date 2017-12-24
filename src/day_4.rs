use std::collections::HashSet;

const PASSPHRASE_SPLIT_WITH: &str = " ";

/// Returns whether or not a given passphrase is valid
/// (contains no repititions)
///
/// # Examples
/// ```
/// # use aoc_2017::day_4::*;
/// assert_eq!(is_valid_passphrase("aa bb cc dd ee"), true);
/// assert_eq!(is_valid_passphrase("aa bb cc dd aa"), false);
/// assert_eq!(is_valid_passphrase("aa bb cc dd aaa"), true);
/// ```
pub fn is_valid_passphrase(passphrase: &str) -> bool {
    let mut lower_case_words = HashSet::new();
    let split_passphrase: Vec<_> = passphrase
        .split(PASSPHRASE_SPLIT_WITH)
        .map(|s| s.trim())
        .collect();
    let passphrase_word_count = split_passphrase.len();
    for w in split_passphrase {
        lower_case_words.insert(w);
    }
    passphrase_word_count == lower_case_words.len()
}
