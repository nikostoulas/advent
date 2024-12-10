use parser::MultiLineParser;

pub fn part1(input: String) -> String {
    let equations = parse_input(input);
    equations
        .iter()
        .filter(|e| can_be_verified(e))
        .map(|e| e[0])
        .sum::<i64>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let equations = parse_input(input);
    equations
        .iter()
        .filter(|e| can_be_verified(e) || can_be_verified_part2(e))
        .map(|e| e[0])
        .sum::<i64>()
        .to_string()
}

fn parse_input(input: String) -> Vec<Vec<i64>> {
    let mut parser = MultiLineParser::new(&input);
    let mut equations: Vec<Vec<i64>> = parser
        .match_number_up_to(':')
        .iter()
        .map(|num| vec![num.unwrap()])
        .collect();

    parser.advance_all_lines(1);

    parser
        .split_to_numbers(" ")
        .iter()
        .enumerate()
        .for_each(|(i, nums)| nums.iter().for_each(|num| equations[i].push(*num)));

    equations
}

fn can_be_verified(nums: &[i64]) -> bool {
    if nums.len() == 2 {
        nums[0] == nums[1]
    } else {
        let remaining = &nums[3..];
        let mut sum_nums = vec![nums[0], nums[1] + nums[2]];
        sum_nums.extend_from_slice(remaining);
        let mut mul_nums = vec![nums[0], nums[1] * nums[2]];
        mul_nums.extend_from_slice(remaining);
        can_be_verified(&sum_nums) || can_be_verified(&mul_nums)
    }
}
fn can_be_verified_part2(nums: &[i64]) -> bool {
    if nums.len() == 2 {
        nums[0] == nums[1]
    } else {
        let remaining = &nums[3..];
        let mut sum_nums = vec![nums[0], nums[1] + nums[2]];
        sum_nums.extend_from_slice(remaining);
        let mut mul_nums = vec![nums[0], nums[1] * nums[2]];
        mul_nums.extend_from_slice(remaining);
        let mut concat_nums = vec![
            nums[0],
            (nums[1].to_string() + &nums[2].to_string())
                .parse()
                .unwrap(),
        ];
        concat_nums.extend_from_slice(remaining);
        can_be_verified_part2(&sum_nums)
            || can_be_verified_part2(&mul_nums)
            || can_be_verified_part2(&concat_nums)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn test_part1() {
        let result = part1(INPUT.to_string());
        assert_eq!(result, "3749");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT.to_string());
        assert_eq!(result, "11387");
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part1_input() {
        assert_eq!(
            part1(include_str!("../../.data/y2024/d7.txt").to_string()),
            "66343330034722"
        );
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part2_input() {
        assert_eq!(
            part2(include_str!("../../.data/y2024/d7.txt").to_string()),
            "637696070419031"
        );
    }
}
