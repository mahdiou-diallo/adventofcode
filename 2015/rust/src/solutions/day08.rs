use std::fs;

pub fn solve() -> (i32, i32) {
    let text = fs::read_to_string("../data/08.txt").unwrap();
    (part1(&text), part2(&text))
}

fn part1(text: &str) -> i32 {
    text.lines()
        .map(|line| line.len() as i32 - count_chars(line))
        .sum::<i32>()
}

fn part2(text: &str) -> i32 {
    text.lines()
        .map(|line| count_encoded_chars(line) - line.len() as i32)
        .sum::<i32>()
}

fn count_chars(line: &str) -> i32 {
    line.chars()
        .fold(CharCounter::new(), |counter, c| counter.next(c))
        .count
}

fn count_encoded_chars(line: &str) -> i32 {
    2 + line
        .chars()
        .fold(EncodedCharCounter::new(), |counter, c| counter.next(c))
        .count
}

enum State {
    Start,
    Normal,
    Escape,
    Hex(u8),
    End,
}

struct CharCounter {
    count: i32,
    state: State,
}

impl CharCounter {
    fn new() -> Self {
        Self {
            count: 0,
            state: State::Start,
        }
    }
    fn next(self, c: char) -> Self {
        match self.state {
            State::Start => match c {
                '"' => Self {
                    count: 0,
                    state: State::Normal,
                },
                _ => unreachable!(),
            },
            State::Normal => match c {
                '\\' => Self {
                    count: self.count,
                    state: State::Escape,
                },
                '"' => Self {
                    count: self.count,
                    state: State::End,
                },
                _ => Self {
                    count: self.count + 1,
                    state: self.state,
                },
            },
            State::Escape => match c {
                'x' => Self {
                    count: self.count,
                    state: State::Hex(0),
                },
                _ => Self {
                    count: self.count + 1,
                    state: State::Normal,
                },
            },
            State::Hex(idx) => {
                if idx == 0 {
                    Self {
                        count: self.count,
                        state: State::Hex(1),
                    }
                } else {
                    Self {
                        count: self.count + 1,
                        state: State::Normal,
                    }
                }
            }
            State::End => self,
        }
    }
}

struct EncodedCharCounter {
    count: i32,
    state: State,
}

impl EncodedCharCounter {
    fn new() -> Self {
        Self {
            count: 0,
            state: State::Start,
        }
    }
    fn next(self, c: char) -> Self {
        match self.state {
            State::Start => match c {
                '"' => Self {
                    count: 2,
                    state: State::Normal,
                },
                _ => unreachable!(),
            },
            State::Normal => match c {
                '\\' => Self {
                    count: self.count + 2,
                    state: State::Escape,
                },
                '"' => Self {
                    count: self.count + 2,
                    state: State::End,
                },
                _ => Self {
                    count: self.count + 1,
                    state: self.state,
                },
            },
            State::Escape => match c {
                'x' => Self {
                    count: self.count + 1,
                    state: State::Hex(0),
                },
                '"' | '\\' => Self {
                    count: self.count + 2,
                    state: State::Normal,
                },
                _ => Self {
                    count: self.count + 1,
                    state: State::Normal,
                },
            },
            State::Hex(idx) => {
                if idx == 0 {
                    Self {
                        count: self.count + 1,
                        state: State::Hex(1),
                    }
                } else {
                    Self {
                        count: self.count + 1,
                        state: State::Normal,
                    }
                }
            }
            State::End => self,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = r#"""
"abc"
"aaa\"aaa"
"\x27""#;

    #[test]
    fn part1_empty_string() {
        assert_eq!(count_chars(r#""""#), 0);
    }

    #[test]
    fn part1_three_chars() {
        assert_eq!(count_chars(r#""abc""#), 3);
    }

    #[test]
    fn part1_one_escape() {
        assert_eq!(count_chars(r#""aaa\"aaa""#), 7);
    }

    #[test]
    fn part1_unicode_escape() {
        assert_eq!(count_chars(r#""\x27""#), 1);
    }

    #[test]
    fn part1_full() {
        assert_eq!(part1(TEST_INPUT), 12);
    }

    #[test]
    fn part2_empty_string() {
        assert_eq!(count_encoded_chars(r#""""#), 6);
    }

    #[test]
    fn part2_three_chars() {
        assert_eq!(count_encoded_chars(r#""abc""#), 9);
    }

    #[test]
    fn part2_one_escape() {
        assert_eq!(count_encoded_chars(r#""aaa\"aaa""#), 16);
    }

    #[test]
    fn part2_unicode_escape() {
        assert_eq!(count_encoded_chars(r#""\x27""#), 11);
    }

    #[test]
    fn part2_full() {
        assert_eq!(part2(TEST_INPUT), 19);
    }
}
