use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::fs;

pub fn solve() -> (i32, i32) {
    let text = fs::read_to_string("../data/01.txt").unwrap();
    (part1(&text), part2(&text))
}

fn part1(text: &str) -> i32 {
    text.chars().fold(0, |acc, c| match c {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => unreachable!(),
    })
}

// fn day1_part2(text: &str) -> i32 {
//     let mut idx = 0;
//     let mut floor = 0;
//     for c in text.chars() {
//         idx += 1;
//         match c {
//             '(' => {
//                 floor += 1;
//             }
//             ')' => {
//                 floor -= 1;
//                 if floor == -1 {
//                     return idx;
//                 }
//             }
//             _ => unreachable!(),
//         }
//     }
//     -1
// }

fn part2(text: &str) -> i32 {
    text.chars()
        .enumerate()
        .fold_while(0, |acc, (i, c)| match c {
            '(' => Continue(acc + 1),
            ')' => {
                if acc == 0 {
                    Done(i as i32 + 1)
                } else {
                    Continue(acc - 1)
                }
            }
            _ => unreachable!(),
        })
        .into_inner()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_return_to_0() {
        assert_eq!(part1("(())"), 0);
        assert_eq!(part1("()()"), 0);
    }

    #[test]
    fn test_go_to_3() {
        assert_eq!(part1("((("), 3);
        assert_eq!(part1("(()(()("), 3);
        assert_eq!(part1("))((((("), 3);
    }

    #[test]
    fn test_go_to_b1() {
        assert_eq!(part1("())"), -1);
        assert_eq!(part1("))("), -1);
    }

    #[test]
    fn test_go_to_b3() {
        assert_eq!(part1(")))"), -3);
        assert_eq!(part1(")())())"), -3);
    }

    #[test]
    fn test_b1_step3() {
        assert_eq!(part2("())"), 3);
    }

    #[test]
    fn test_b1_step1() {
        assert_eq!(part2(")))"), 1);
    }
}
