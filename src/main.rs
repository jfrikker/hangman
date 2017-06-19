use std::hash::Hash;
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
    let counts: Counted<char> = DICT.lines()
        .filter(|w| pattern.matches(w))
        .flat_map(|w| completions(pattern, w))
        .collect();

    counts.desc()
}

struct Counted<T> {
    counts: HashMap<T, u16>
}

impl<T> Counted<T>
    where T: Hash + Eq + Copy {
    fn new() -> Counted<T> {
        Counted {
            counts: HashMap::new()
        }
    }

    fn inc(&mut self, key: T) {
        let count = self.counts.entry(key).or_insert(0);
        *count += 1;
    }

    fn desc(mut self) -> Vec<T> {
        let mut pairs: Vec<(T, u16)> = self.counts.drain()
            .collect();
        pairs.sort_by(|&(_, count1), &(_, count2)| count2.cmp(&count1));

        pairs.iter()
            .map(|&(c, _)| c)
            .collect()
    }
}

impl<T> FromIterator<T> for Counted<T>
    where T: Hash + Eq + Copy {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut counts = Counted::new();

        for item in iter {
            counts.inc(item);
        }

        counts
    }
}

fn completions(pattern: &Pattern, candidate: &str) -> HashSet<char> {
    pattern.pattern.chars().zip(candidate.chars())
        .filter(|&(p, _)| p == '_')
        .map(|(_, c)| c)
        .collect()
}
