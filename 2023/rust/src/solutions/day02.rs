use std::fs;

use regex::Regex;

pub fn solve() -> (i32, i32) {
    let text = fs::read_to_string("../data/02.txt").unwrap();
    (part1(&text), part2(&text))
}

fn part1(text: &str) -> i32 {
    let p = Parser::new();
    text.lines()
        .map(|line| get_max_counts(line, &p))
        .filter(|(_, Counts { red, green, blue })| *red <= 12 && *green <= 13 && *blue <= 14)
        .map(|(id, _)| id)
        .sum()
}

fn part2(text: &str) -> i32 {
    let p = Parser::new();
    text.lines()
        .map(|line| get_max_counts(line, &p))
        .map(|(_, Counts { red, green, blue })| red * blue * green)
        .sum()
}

fn get_max_counts(line: &str, p: &Parser) -> (i32, Counts) {
    p.parse(line)
}

struct Parser {
    re_game_id: Regex,
    re_color_count: Regex,
}

#[derive(PartialEq, Eq, Debug)]
struct Counts {
    red: i32,
    green: i32,
    blue: i32,
}

impl Counts {
    fn new() -> Self {
        Counts {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

impl Parser {
    fn new() -> Self {
        Parser {
            re_game_id: Regex::new(r"^Game (\d+):\s+").unwrap(),
            re_color_count: Regex::new(r"(\d+) (red|green|blue)").unwrap(),
        }
    }

    fn parse(&self, txt: &str) -> (i32, Counts) {
        let id_capture = self.re_game_id.captures(txt).unwrap();

        let info_start = id_capture.get(0).unwrap().end();
        let id = id_capture.get(1).unwrap().as_str().parse::<i32>().unwrap();

        let max_counts = self
            .re_color_count
            .captures_iter(&txt[info_start..])
            .map(|capture| {
                (
                    capture.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                    capture.get(2).unwrap().as_str(),
                )
            })
            .fold(
                Counts::new(),
                |Counts { red, green, blue }, curr| match curr {
                    (x, "red") => Counts {
                        red: red.max(x),
                        green,
                        blue,
                    },
                    (x, "green") => Counts {
                        red,
                        green: green.max(x),
                        blue,
                    },
                    (x, "blue") => Counts {
                        red,
                        green,
                        blue: blue.max(x),
                    },
                    _ => unreachable!(),
                },
            );

        (id, max_counts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game1() {
        let (id, counts) = get_max_counts(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            &Parser::new(),
        );

        assert_eq!(id, 1);

        assert_eq!(
            counts,
            Counts {
                red: 4,
                green: 2,
                blue: 6
            }
        );
    }

    #[test]
    fn game2() {
        let (id, counts) = get_max_counts(
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            &Parser::new(),
        );

        assert_eq!(id, 2);

        assert_eq!(
            counts,
            Counts {
                red: 1,
                green: 3,
                blue: 4
            }
        );
    }

    #[test]
    fn game3() {
        let (id, counts) = get_max_counts(
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            &Parser::new(),
        );

        assert_eq!(id, 3);

        assert_eq!(
            counts,
            Counts {
                red: 20,
                green: 13,
                blue: 6
            }
        );
    }

    #[test]
    fn game4() {
        let (id, counts) = get_max_counts(
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            &Parser::new(),
        );

        assert_eq!(id, 4);

        assert_eq!(
            counts,
            Counts {
                red: 14,
                green: 3,
                blue: 15
            }
        );
    }

    #[test]
    fn game5() {
        let (id, counts) = get_max_counts(
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
            &Parser::new(),
        );

        assert_eq!(id, 5);

        assert_eq!(
            counts,
            Counts {
                red: 6,
                green: 3,
                blue: 2
            }
        );
    }

    #[test]
    fn part1_full() {
        assert_eq!(
            part1(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            8
        );
    }

    #[test]
    fn part2_full() {
        assert_eq!(
            part2(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            2286
        );
    }
}
