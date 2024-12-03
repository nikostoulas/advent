use regex::Regex;

pub fn part1(input: String) -> String {
    let nums = parse_input(&input);
    nums.iter().map(|[a, b]| a * b).sum::<i32>().to_string()
}

pub fn part2(input: String) -> String {
    let nums = parse_input_2(&input);
    nums.iter().map(|[a, b]| a * b).sum::<i32>().to_string()
}

fn parse_input(input: &str) -> Vec<[i32; 2]> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut results = vec![];
    for (_, [num1, num2]) in re.captures_iter(input).map(|c| c.extract()) {
        results.push([num1.parse::<i32>().unwrap(), num2.parse::<i32>().unwrap()]);
    }
    results
}

fn parse_input_2(mut input: &str) -> Vec<[i32; 2]> {
    let mut result = vec![];
    while !input.is_empty() {
        let (before_dont, after_dont) = split(input, "don't()");
        result.append(&mut parse_input(before_dont));
        input = split(after_dont, "do()").1;
    }
    result
}

fn split<'a>(input: &'a str, up_to: &'a str) -> (&'a str, &'a str) {
    input.split_once(up_to).unwrap_or((input, ""))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = part1(str.to_string());
        assert_eq!(result, "161");
        assert_eq!(
            part1(include_str!("../../.data/y2024/d3.txt").to_string()),
            "184122457"
        );
    }

    #[test]
    fn test_part2() {
        let str = "don't()do()xmul(2,4)&mul[3,7]!^don't()_don't()mul(5,5)+mul(32,64](mul(11,8)undo()do()?mul(8,5))don't()mul(4,2)testd()mul(8,1)don't()mul(4,1)d()mul(8,1)";
        let result = part2(str.to_string());
        assert_eq!(result, "48");
        assert_eq!(
            part2(include_str!("../../.data/y2024/d3.txt").to_string()),
            "107862689"
        );
    }
}
