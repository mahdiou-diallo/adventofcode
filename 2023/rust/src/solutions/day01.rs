use std::fs;

use regex::Regex;

pub fn solve() -> (i32, i32) {
    let text = fs::read_to_string("../data/01.txt").unwrap();
    (part1(&text), part2(&text))
}

fn part1(text: &str) -> i32 {
    text.lines().map(parse_number).sum()
}

fn part2(text: &str) -> i32 {
    let p = Parser::new();
    text.lines().map(|line| parse_number2(line, &p)).sum()
}

fn parse_number(line: &str) -> i32 {
    let c1 = get_first_digit(line.chars());
    let c2 = get_first_digit(line.chars().rev());

    c1 * 10 + c2
}

fn get_first_digit(it: impl Iterator<Item = char>) -> i32 {
    it.skip_while(|x| !x.is_numeric())
        .next()
        .unwrap()
        .to_digit(10)
        .unwrap() as i32
}

fn parse_number2(line: &str, p: &Parser) -> i32 {
    p.parse(line)
}

fn to_digit(number: &str) -> i32 {
    match number {
        x if x.len() == 1 => x.chars().next().unwrap().to_digit(10).unwrap() as i32,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => unreachable!(),
    }
}

struct Parser {
    re: Regex,
    re_rev: Regex,
}

impl Parser {
    fn new() -> Self {
        Parser {
            re: Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap(),
            re_rev: Regex::new(r"(\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap(),
        }
    }

    fn parse(&self, txt: &str) -> i32 {
        let first_match = self.re.captures(txt).unwrap().get(1).unwrap();

        let first = first_match.as_str();
        let reversed = txt.chars().rev().collect::<String>();
        let last = self
            .re_rev
            .captures(&reversed)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .chars()
            .rev()
            .collect::<String>();

        10 * to_digit(first) + to_digit(&last)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_last() {
        assert_eq!(parse_number("1abc2"), 12);
    }

    #[test]
    fn inner_digits() {
        assert_eq!(parse_number("pqr3stu8vwx"), 38);
    }

    #[test]
    fn multiple_digits() {
        assert_eq!(parse_number("a1b2c3d4e5f"), 15);
    }

    #[test]
    fn single_digit() {
        assert_eq!(parse_number("treb7uchet"), 77);
    }

    #[test]
    fn part1_full() {
        assert_eq!(
            part1(
                "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            ),
            142
        );
    }

    #[test]
    fn two_all_letters() {
        assert_eq!(parse_number2("two1nine", &Parser::new()), 29);
    }

    #[test]
    fn overlapping() {
        assert_eq!(parse_number2("twone", &Parser::new()), 21);
    }

    #[test]
    fn p2_sample1() {
        assert_eq!(parse_number2("eightwothree", &Parser::new()), 83);
    }

    #[test]
    fn p2_sample2() {
        assert_eq!(parse_number2("abcone2threexyz", &Parser::new()), 13);
    }

    #[test]
    fn p2_sample3() {
        assert_eq!(parse_number2("xtwone3four", &Parser::new()), 24);
    }

    #[test]
    fn p2_sample4() {
        assert_eq!(parse_number2("4nineeightseven2", &Parser::new()), 42);
    }

    #[test]
    fn p2_sample5() {
        assert_eq!(parse_number2("zoneight234", &Parser::new()), 14);
    }

    #[test]
    fn p2_sample6() {
        assert_eq!(parse_number2("7pqrstsixteen", &Parser::new()), 76);
    }

    #[test]
    fn part2_full() {
        assert_eq!(
            part2(
                "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            ),
            281
        );
    }
}
