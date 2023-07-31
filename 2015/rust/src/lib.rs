use std::fs;

struct Solution;

fn read_input(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

pub fn day1() -> (i32, i32) {
    let text = read_input("../data/01.txt");
    (Solution::day1_part1(&text), Solution::day1_part2(&text))
}

struct State {
    idx: i32,
    floor: i32,
}
impl Solution {
    fn day1_part1(text: &str) -> i32 {
        text.chars().fold(0, |acc, c| match c {
            '(' => acc + 1,
            ')' => acc - 1,
            _ => unreachable!(),
        })
    }

    fn day1_part2(text: &str) -> i32 {
        let mut acc = State { idx: 0, floor: 0 };
        for c in text.chars() {
            acc.idx += 1;
            match c {
                '(' => {
                    acc.floor += 1;
                }
                ')' => {
                    acc.floor -= 1;
                    if acc.floor == -1 {
                        return acc.idx;
                    }
                }
                _ => unreachable!(),
            }
        }
        -1
    }

    // fn day1_part2(text: &str) -> i32 {
    //     let r = text
    //         .chars()
    //         .scan(State { idx: 0, floor: 0 }, |acc, c| {
    //             acc.idx += 1;
    //             match c {
    //                 '(' => {
    //                     acc.floor += 1;
    //                     Some(acc.idx)
    //                 }
    //                 ')' => {
    //                     acc.floor -= 1;
    //                     if acc.floor == -1 {
    //                         None
    //                     } else {
    //                         Some(acc.idx)
    //                     }
    //                 }
    //                 _ => unreachable!(),
    //             }
    //         })
    //         .collect::<Vec<_>>();
    //     println!("{:?}", r);
    //     0
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_return_to_0() {
        assert_eq!(Solution::day1_part1("(())"), 0);
        assert_eq!(Solution::day1_part1("()()"), 0);
    }

    #[test]
    fn test_go_to_3() {
        assert_eq!(Solution::day1_part1("((("), 3);
        assert_eq!(Solution::day1_part1("(()(()("), 3);
        assert_eq!(Solution::day1_part1("))((((("), 3);
    }

    #[test]
    fn test_go_to_b1() {
        assert_eq!(Solution::day1_part1("())"), -1);
        assert_eq!(Solution::day1_part1("))("), -1);
    }

    #[test]
    fn test_go_to_b3() {
        assert_eq!(Solution::day1_part1(")))"), -3);
        assert_eq!(Solution::day1_part1(")())())"), -3);
    }

    #[test]
    fn test_b1_step3() {
        assert_eq!(Solution::day1_part2("())"), 3);
    }

    #[test]
    fn test_b1_step1() {
        assert_eq!(Solution::day1_part2(")))"), 1);
    }
}
