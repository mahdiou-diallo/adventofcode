use regex::Regex;
use std::fs;

const GRID_SIZE: usize = 1000;

pub fn solve() -> (i32, i32) {
    let text = fs::read_to_string("../data/06.txt").unwrap();
    (part1(&text), part2(&text))
}

fn part1(text: &str) -> i32 {
    count_lights(text, Box::new(BooleanGrid::new()))
}

fn part2(text: &str) -> i32 {
    count_lights(text, Box::new(CounterGrid::new()))
}

fn count_lights(text: &str, grid: Box<dyn Grid>) -> i32 {
    let parser = Parser::new();
    text.lines()
        .fold(grid, |mut grid, line| {
            grid.update(&parser.parse_command(line));
            grid
        })
        .count_lights()
}

trait Grid {
    fn count_lights(&self) -> i32;
    fn update(&mut self, action: &Action);
}

struct BooleanGrid {
    g: Vec<Vec<bool>>,
}

impl BooleanGrid {
    fn new() -> Self {
        BooleanGrid {
            g: (0..GRID_SIZE)
                .map(|_| (0..GRID_SIZE).map(|_| false).collect())
                .collect(),
        }
    }

    fn turn_on(&mut self, from: &(usize, usize), to: &(usize, usize)) {
        for i in from.0..=to.0 {
            for j in from.1..=to.1 {
                self.g[i][j] = true
            }
        }
    }

    fn turn_off(&mut self, from: &(usize, usize), to: &(usize, usize)) {
        for i in from.0..=to.0 {
            for j in from.1..=to.1 {
                self.g[i][j] = false
            }
        }
    }

    fn toggle(&mut self, from: &(usize, usize), to: &(usize, usize)) {
        for i in from.0..=to.0 {
            for j in from.1..=to.1 {
                self.g[i][j] = !self.g[i][j]
            }
        }
    }
}

impl Grid for BooleanGrid {
    fn update(&mut self, action: &Action) {
        match action {
            Action::On { from, to } => self.turn_on(from, to),
            Action::Off { from, to } => self.turn_off(from, to),
            Action::Toggle { from, to } => self.toggle(from, to),
        }
    }

    fn count_lights(&self) -> i32 {
        self.g
            .iter()
            .map(|row| row.iter().filter(|item| **item).count() as i32)
            .sum()
    }
}

struct CounterGrid {
    g: Vec<Vec<u32>>,
}

impl CounterGrid {
    fn new() -> Self {
        CounterGrid {
            g: (0..GRID_SIZE)
                .map(|_| (0..GRID_SIZE).map(|_| 0).collect())
                .collect(),
        }
    }

    fn turn_on(&mut self, from: &(usize, usize), to: &(usize, usize)) {
        for i in from.0..=to.0 {
            for j in from.1..=to.1 {
                self.g[i][j] += 1
            }
        }
    }

    fn turn_off(&mut self, from: &(usize, usize), to: &(usize, usize)) {
        for i in from.0..=to.0 {
            for j in from.1..=to.1 {
                self.g[i][j] = u32::saturating_sub(self.g[i][j], 1)
            }
        }
    }

    fn toggle(&mut self, from: &(usize, usize), to: &(usize, usize)) {
        for i in from.0..=to.0 {
            for j in from.1..=to.1 {
                self.g[i][j] = self.g[i][j] + 2
            }
        }
    }
}

impl Grid for CounterGrid {
    fn update(&mut self, action: &Action) {
        match action {
            Action::On { from, to } => self.turn_on(from, to),
            Action::Off { from, to } => self.turn_off(from, to),
            Action::Toggle { from, to } => self.toggle(from, to),
        }
    }

    fn count_lights(&self) -> i32 {
        self.g
            .iter()
            .map(|row| row.iter().map(|item| *item as i32).sum::<i32>())
            .sum::<i32>()
    }
}

#[derive(Debug)]
enum Action {
    Toggle {
        from: (usize, usize),
        to: (usize, usize),
    },
    On {
        from: (usize, usize),
        to: (usize, usize),
    },
    Off {
        from: (usize, usize),
        to: (usize, usize),
    },
}

struct Parser {
    re: Regex,
}

impl Parser {
    fn new() -> Self {
        Parser {
            re: Regex::new(r"(?<action>[a-z]+(?: [a-z]+)?) (?<f_s>\d+),(?<f_e>\d+) through (?<t_s>\d+),(?<t_e>\d+)").unwrap(),
        }
    }

    fn parse_command(&self, cmd: &str) -> Action {
        let caps = self.re.captures(cmd).unwrap();

        let from_start: usize = caps.get(2).unwrap().as_str().parse().unwrap();
        let from_end: usize = caps.get(3).unwrap().as_str().parse().unwrap();
        let to_start: usize = caps.get(4).unwrap().as_str().parse().unwrap();
        let to_end: usize = caps.get(5).unwrap().as_str().parse().unwrap();
        let from = (from_start, from_end);

        let to = (to_start, to_end);

        match caps.get(1).unwrap().as_str() {
            "toggle" => Action::Toggle { from, to },
            "turn on" => Action::On { from, to },
            "turn off" => Action::Off { from, to },
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn million_lights() {
        assert_eq!(part1("turn on 0,0 through 999,999"), 1000000);
    }

    #[test]
    fn first_row() {
        assert_eq!(part1("toggle 0,0 through 999,0"), 1000);
    }

    #[test]
    fn first_light_on_once() {
        assert_eq!(part2("turn on 0,0 through 0,0"), 1);
    }

    #[test]
    fn all_lights_toggle_once() {
        assert_eq!(part2("toggle 0,0 through 999,999"), 2000000);
    }
}
