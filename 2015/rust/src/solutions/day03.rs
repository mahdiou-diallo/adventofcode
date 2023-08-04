use std::collections::HashMap;
use std::fs;

pub fn solve() -> (i32, i32) {
    let text = fs::read_to_string("../data/03.txt").unwrap();
    (part1(&text), part2(&text))
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct Pos(i32, i32);

fn part1(text: &str) -> i32 {
    text.chars()
        .fold(
            (HashMap::from([(Pos(0, 0), true)]), Pos(0, 0)),
            |(mut visited, pos), c| {
                let (dx, dy) = match c {
                    '^' => (0, 1),
                    '>' => (1, 0),
                    'v' => (0, -1),
                    '<' => (-1, 0),
                    _ => unreachable!(),
                };
                let p = Pos(pos.0 + dx, pos.1 + dy);
                visited.entry(p).or_insert(true);
                (visited, p)
            },
        )
        .0
        .len() as i32
}

fn part2(text: &str) -> i32 {
    text.chars()
        .enumerate()
        .fold(
            (HashMap::from([(Pos(0, 0), true)]), Pos(0, 0), Pos(0, 0)),
            |(mut visited, pos1, pos2), (idx, c)| {
                let (dx, dy) = match c {
                    '^' => (0, 1),
                    '>' => (1, 0),
                    'v' => (0, -1),
                    '<' => (-1, 0),
                    _ => unreachable!(),
                };

                let bot_1_moves = idx % 2 == 0;
                let pos = if bot_1_moves { pos1 } else { pos2 };

                let p = Pos(pos.0 + dx, pos.1 + dy);

                visited.entry(p).or_insert(true);

                if bot_1_moves {
                    (visited, p, pos2)
                } else {
                    (visited, pos1, p)
                }
            },
        )
        .0
        .len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visit_2() {
        assert_eq!(part1(">"), 2);
    }

    #[test]
    fn test_visit_4() {
        assert_eq!(part1("^>v<"), 4);
    }

    #[test]
    fn test_visit_2_multiple() {
        assert_eq!(part1("^v^v^v^v^v"), 2);
    }

    #[test]
    fn test_2_visit_3() {
        assert_eq!(part2("^v"), 3);
    }

    #[test]
    fn test_2_visit_3_multiple() {
        assert_eq!(part2("^>v<"), 3);
    }

    #[test]
    fn test_2_visit_11() {
        assert_eq!(part2("^v^v^v^v^v"), 11);
    }
}
