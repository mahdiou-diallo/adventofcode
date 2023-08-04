use md5;
use std::fs;

pub fn solve() -> (i32, i32) {
    let text = fs::read_to_string("../data/04.txt").unwrap();
    (part1(&text), part2(&text))
}

fn part1(text: &str) -> i32 {
    (0..=std::i32::MAX)
        .take_while(|i| {
            let plain = format!("{}{}", text, i);
            let digest = format!("{:x}", md5::compute(plain));
            !digest.starts_with("00000")
        })
        .last()
        .unwrap()
        + 1
}

fn part2(text: &str) -> i32 {
    (0..=std::i32::MAX)
        .take_while(|i| {
            let plain = format!("{}{}", text, i);
            let digest = format!("{:x}", md5::compute(plain));
            !digest.starts_with("000000")
        })
        .last()
        .unwrap()
        + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_abcdef() {
        assert_eq!(part1("abcdef"), 609043);
    }

    #[test]
    fn test_key_pqrstuv() {
        assert_eq!(part1("pqrstuv"), 1048970);
    }
}
