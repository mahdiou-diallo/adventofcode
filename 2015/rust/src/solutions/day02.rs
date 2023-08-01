use itertools::sorted;
use itertools::FoldWhile::{Continue, Done};
use std::fs;

pub fn solve() -> (i32, i32) {
    let text = fs::read_to_string("../data/02.txt").unwrap();
    (part1(&text), part2(&text))
}

fn part1(text: &str) -> i32 {
    text.lines()
        .map(|line| {
            let v = line
                .split("x")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let v: [i32; 3] = sorted(v).collect::<Vec<i32>>().try_into().unwrap();
            let [l, w, h] = v;
            2 * l * w + 2 * w * h + 2 * h * l + l * w
        })
        .sum()
}

fn part2(text: &str) -> i32 {
    text.lines()
        .map(|line| {
            let v = line
                .split("x")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let v: [i32; 3] = sorted(v).collect::<Vec<i32>>().try_into().unwrap();
            let [l, w, h] = v;
            2 * l + 2 * w + l * w * h
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_58() {
        assert_eq!(part1("2x3x4"), 58);
    }

    #[test]
    fn test_area_43() {
        assert_eq!(part1("1x1x10"), 43);
    }

    #[test]
    fn test_len_34() {
        assert_eq!(part2("2x3x4"), 34);
    }

    #[test]
    fn test_len_14() {
        assert_eq!(part2("1x1x10"), 14);
    }
}
