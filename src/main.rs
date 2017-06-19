use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;
use std::env::args;
use std::iter::FromIterator;


fn main() {
    let pattern = read_args();
    let guess_dict = best_guess_dictionary(&pattern);

    println!("{}", guess_dict.get(0).map_or(String::from("Guess at random"),
                                  |c| c.to_string()));
}

struct Pattern {
    pattern: String,
    seen: HashSet<char>
}

impl Pattern {
    fn matches(&self, candidate: &str) -> bool {
        if candidate.len() != self.pattern.len() {
            return false;
        }

        self.pattern.chars().zip(candidate.chars())
            .all(|(p, c)| p == c ||
                 (p == '_' && !self.seen.contains(&c)))
    }
}

fn read_args() -> Pattern {
    let pattern = args().nth(1).unwrap();
    let guessed = args().nth(2).unwrap_or(String::new());
    let mut guessed_set = HashSet::from_iter(guessed.chars());
    guessed_set.extend(pattern.chars()
                        .filter(|&c| c != '_'));

    Pattern {
        pattern: pattern,
        seen: guessed_set
    }
}

const DICT: &str = include_str!("dict.txt");

fn best_guess_dictionary(pattern: &Pattern) -> Vec<char> {
    let mut counts: HashMap<char, u16> = HashMap::new();
    for word in DICT.lines() {
        if pattern.matches(word) {
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

fn completions(pattern: &Pattern, candidate: &str) -> HashSet<char> {
    let options = pattern.pattern.chars().zip(candidate.chars())
        .filter(|&(p, _)| p == '_')
        .map(|(_, c)| c);
    HashSet::from_iter(options)
}
