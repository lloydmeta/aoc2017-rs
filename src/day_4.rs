use std::collections::{HashMap, HashSet};

const DAY_4_INPUT: &str = include_str!("../data/day_4_input");
const PASSPHRASE_SPLIT_WITH: &str = " ";

pub fn run() -> Result<(), &'static str> {
    println!("*** Day 4: High-Entropy Passphrases ***");
    println!("Input: {}", DAY_4_INPUT);
    let passphrases: Vec<_> = DAY_4_INPUT.trim().split("\n").collect();
    let valid_passphrases_1 = passphrases
        .iter()
        .filter(|s| are_valid_passphrases(s))
        .count();
    let valid_passphrases_2 = passphrases
        .iter()
        .filter(|s| are_valid_passphrases_annagram_free(s))
        .count();
    println!("Solution 1: {}\n", valid_passphrases_1);
    println!("Solution 2: {}\n", valid_passphrases_2);
    Ok(())
}

fn are_valid_passphrases<'a>(passphrases_str: &'a str) -> bool {
    let h_set: HashSet<&'a str> = HashSet::new();
    are_valid_passphrases_inner(passphrases_str, h_set)
}

fn are_valid_passphrases_annagram_free<'a>(passphrases_str: &'a str) -> bool {
    let h_set: HashSet<Vec<(char, u64)>> = HashSet::new();
    are_valid_passphrases_inner(passphrases_str, h_set)
}

fn are_valid_passphrases_inner<'a, V>(passphrases_str: &'a str, validator: V) -> bool
where
    V: PassphrasesValidator<'a>,
{
    let passphrases: Vec<_> = passphrases_str
        .split(PASSPHRASE_SPLIT_WITH)
        .map(|s| s.trim())
        .collect();
    let validated = passphrases.iter().fold(validator, |mut acc, next| {
        acc.register_phrase(next);
        acc
    });
    validated.validate_phrases(&passphrases)
}

trait PassphrasesValidator<'a> {
    fn register_phrase(&mut self, s: &'a str);
    fn validate_phrases(&self, s: &Vec<&str>) -> bool;
}

impl<'a> PassphrasesValidator<'a> for HashSet<&'a str> {
    fn register_phrase(&mut self, s: &'a str) -> () {
        self.insert(s);
    }
    fn validate_phrases(&self, s: &Vec<&str>) -> bool {
        self.len() == s.len()
    }
}

impl<'a> PassphrasesValidator<'a> for HashSet<Vec<(char, u64)>> {
    fn register_phrase(&mut self, s: &'a str) -> () {
        let char_occurences = s.chars()
            .fold(HashMap::with_capacity(s.len()), |mut acc, next| {
                *acc.entry(next).or_insert(0) += 1;
                acc
            });
        let mut as_v: Vec<_> = char_occurences.into_iter().collect();
        as_v.sort_by(|&(c1, _), &(c2, _)| c1.cmp(&c2));
        self.insert(as_v);
    }
    fn validate_phrases(&self, s: &Vec<&str>) -> bool {
        self.len() == s.len()
    }
}
