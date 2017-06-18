use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;
use std::env::args;
use std::iter::FromIterator;


fn main() {
    let (pattern, guessed) = read_args();
    let guess_dict = best_guess_dictionary(&pattern);
    let mut guess_dict_filtered = guess_dict
        .iter()
        .filter(|c| !guessed.contains(c));

    println!("{}", guess_dict_filtered.next().map_or(String::from("Guess at random"),
                                  |c| c.to_string()));
}

fn read_args() -> (String,  HashSet<char>) {
    let pattern = args().nth(1).unwrap();
    let guessed = args().nth(2).unwrap_or(String::new());
    let mut guessed_set = HashSet::from_iter(guessed.chars());
    guessed_set.extend(pattern.chars()
                        .filter(|&c| c != '_'));

    (pattern, guessed_set)
}

const DICT: &str = include_str!("dict.txt");

fn best_guess_dictionary(pattern: &str) -> Vec<char> {
    let mut counts: HashMap<char, u16> = HashMap::new();
    for word in DICT.lines() {
        if matches_pattern(pattern, word) {
            for c in completions(pattern, word) {
                let count = counts.entry(c).or_insert(0);
                *count += 1;
            }
        }
    }

    let mut pairs: Vec<(char, u16)> = counts.drain()
        .collect();
    pairs.sort_by(|&(_, count1), &(_, count2)| count2.cmp(&count1));

    pairs.iter()
        .map(|&(c, _)| c)
        .collect()
}

fn matches_pattern(pattern: &str, candidate: &str) -> bool {
    if candidate.len() != pattern.len() {
        return false;
    }

    for (p, c) in pattern.chars().zip(candidate.chars()) {
        if p != '_' &&
            p != c {
            return false;
        }
    }

    true
}

fn completions(pattern: &str, candidate: &str) -> HashSet<char> {
    let options = pattern.chars().zip(candidate.chars())
        .filter(|&(p, _)| p == '_')
        .map(|(_, c)| c);
    HashSet::from_iter(options)
}
