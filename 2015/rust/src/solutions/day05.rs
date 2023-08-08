use itertools::{
    FoldWhile::{Continue, Done},
    Itertools,
};
use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn solve() -> (i32, i32) {
    let text = fs::read_to_string("../data/05.txt").unwrap();

    (part1(&text), part2(&text))
}

fn part1(text: &str) -> i32 {
    let naughty_pairs = get_naughty_pairs();
    text.lines()
        .map(|t| is_nice(t, &naughty_pairs))
        .filter(|item| *item)
        .count() as i32
}

fn part2(text: &str) -> i32 {
    text.lines()
        .map(|t| is_nice2(t))
        .filter(|item| *item)
        .count() as i32
}

#[derive(Debug)]
struct State {
    vowel_count: u32,
    has_double_letter: bool,
    has_naughty_pair: bool,
    char_n_1: Option<char>,
}

impl State {
    fn new() -> State {
        State {
            vowel_count: 0,
            has_double_letter: false,
            has_naughty_pair: false,
            char_n_1: None,
        }
    }
}

fn get_naughty_pairs() -> HashSet<&'static str> {
    HashSet::from(["ab", "cd", "pq", "xy"])
}

fn is_nice(text: &str, naughty_pairs: &HashSet<&str>) -> bool {
    let state = text
        .chars()
        .enumerate()
        .fold_while(
            State::new(),
            |State {
                 char_n_1,
                 mut has_naughty_pair,
                 mut vowel_count,
                 mut has_double_letter,
             },
             (i, c)| {
                if "aeiou".contains(c) {
                    vowel_count += 1;
                }
                if !has_naughty_pair && i > 0 && naughty_pairs.contains(&text[i - 1..=i]) {
                    has_naughty_pair = true;
                }
                if !has_double_letter && char_n_1.is_some() && c == char_n_1.unwrap() {
                    has_double_letter = true
                }

                let s = State {
                    has_naughty_pair,
                    has_double_letter,
                    vowel_count,
                    char_n_1: Some(c),
                };

                if has_naughty_pair {
                    Done(s)
                } else {
                    Continue(s)
                }
            },
        )
        .into_inner();
    !state.has_naughty_pair && state.has_double_letter && state.vowel_count >= 3
}

#[derive(Debug)]
struct State2<'a> {
    char_n_2: Option<char>,
    char_n_1: Option<char>,
    has_repeat_pair: bool,
    has_3_symmetry: bool,
    pairs: HashMap<&'a str, usize>,
}

impl<'a> State2<'a> {
    fn new() -> State2<'a> {
        State2 {
            char_n_2: None,
            char_n_1: None,
            has_repeat_pair: false,
            has_3_symmetry: false,
            pairs: HashMap::new(),
        }
    }
}

fn is_nice2(text: &str) -> bool {
    let state = text
        .chars()
        .enumerate()
        .fold_while(
            State2::new(),
            |State2 {
                 char_n_2,
                 char_n_1,
                 mut has_repeat_pair,
                 mut has_3_symmetry,
                 mut pairs,
             },
             (i, c)| {
                if !has_3_symmetry && char_n_2.is_some() && c == char_n_2.unwrap() {
                    has_3_symmetry = true
                }
                if !has_repeat_pair && i > 0 {
                    let pair = &text[i - 1..=i];
                    match pairs.get(pair) {
                        Some(pair_idx) => {
                            if *pair_idx < i - 1 {
                                has_repeat_pair = true
                            }
                        }
                        None => _ = pairs.insert(pair, i),
                    }
                }

                let state = State2 {
                    char_n_2: char_n_1,
                    char_n_1: Some(c),
                    has_3_symmetry,
                    has_repeat_pair,
                    pairs,
                };
                if has_3_symmetry && has_repeat_pair {
                    Done(state)
                } else {
                    Continue(state)
                }
            },
        )
        .into_inner();
    state.has_3_symmetry && state.has_repeat_pair
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_nice_1() {
        assert_eq!(is_nice("ugknbfddgicrmopn", &get_naughty_pairs()), true);
    }

    #[test]
    fn part_1_is_nice_2() {
        assert_eq!(is_nice("aaa", &get_naughty_pairs()), true);
    }

    #[test]
    fn part_1_is_naughty_1() {
        assert_eq!(is_nice("jchzalrnumimnmhp", &get_naughty_pairs()), false);
    }

    #[test]
    fn part_1_is_naughty_2() {
        assert_eq!(is_nice("haegwjzuvuyypxyu", &get_naughty_pairs()), false);
    }

    #[test]
    fn part_1_is_naughty_3() {
        assert_eq!(is_nice("dvszwmarrgswjxmb", &get_naughty_pairs()), false);
    }

    #[test]
    fn part_2_nice_1() {
        assert_eq!(is_nice2("qjhvhtzxzqqjkmpb"), true);
    }

    #[test]
    fn part_2_nice_2() {
        assert_eq!(is_nice2("xxyxx"), true);
    }

    #[test]
    fn part_2_naughty_1() {
        assert_eq!(is_nice2("uurcxstgmygtbstg"), false);
    }

    #[test]
    fn part_2_naughty_2() {
        assert_eq!(is_nice2("ieodomkazucvgmuy"), false);
    }
}
