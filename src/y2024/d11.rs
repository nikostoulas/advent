use cached::proc_macro::cached;

use parser::Parser;

pub fn part1(input: String) -> String {
    let stones = Parser::new(&input).split_to_numbers(" ");
    let sum: usize = stones.iter().map(|s| blink_n(*s, 25)).sum();
    sum.to_string()
}

pub fn part2(input: String) -> String {
    let stones = Parser::new(&input).split_to_numbers(" ");
    let sum: usize = stones.iter().map(|s| blink_n(*s, 75)).sum();
    sum.to_string()
}

fn blink(num: i64) -> Vec<i64> {
    let str = num.to_string();
    match num {
        0 => vec![1],
        _ if str.len() % 2 == 0 => {
            let left = str[0..str.len() / 2].parse().unwrap();
            let right = str[str.len() / 2..].parse().unwrap_or(0);
            vec![left, right]
        }
        num => vec![num * 2024],
    }
}

#[cached]
fn blink_n(stone_num: i64, num: usize) -> usize {
    let next_stones = blink(stone_num);
    if num == 1 {
        return next_stones.len();
    }
    let map = next_stones.into_iter().map(|s| blink_n(s, num - 1));
    map.sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
125 17";

    #[test]
    fn test_stone_blink() {
        assert_eq!(blink(1000), vec![10, 0]);
        assert_eq!(blink(17), vec![1, 7]);
        assert_eq!(blink(0), vec![1]);
        assert_eq!(blink(1), vec![2024]);
    }

    #[test]
    fn test_part1() {
        let result = part1(INPUT.to_string());
        assert_eq!(result, "55312");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT.to_string());
        assert_eq!(result, "65601038650482");
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part1_input() {
        assert_eq!(
            part1(include_str!("../../.data/y2024/d11.txt").to_string()),
            "217443"
        );
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part2_input() {
        assert_eq!(
            part2(include_str!("../../.data/y2024/d11.txt").to_string()),
            "257246536026785"
        );
    }
}
