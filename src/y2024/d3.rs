use parser::Parser;
pub fn part1(input: String) -> String {
    let nums = parse_input(&input);
    nums.iter().map(|[a, b]| a * b).sum::<i64>().to_string()
}

pub fn part2(input: String) -> String {
    let nums = parse_input_2(&input);
    nums.iter().map(|[a, b]| a * b).sum::<i64>().to_string()
}

fn parse_input(input: &str) -> Vec<[i64; 2]> {
    let mut parser = Parser::new(input);
    let mut results = vec![];

    while parser.advance_to("mul(") {
        parser.advance(1);
        let num1 = parser.match_number_up_to(',');
        let num2 = parser.match_number_up_to(')');
        results.push([num1.unwrap_or(0), num2.unwrap_or(0)]);
    }
    results
}

fn parse_input_2(input: &str) -> Vec<[i64; 2]> {
    let mut parser = Parser::new(input);
    let input = parser.delete_between("don't()", "do()");
    parse_input(&input)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_part1() {
        let result = part1(INPUT.to_string());
        assert_eq!(result, "161");
    }

    #[test]
    fn test_part2() {
        let str = "don't()do()xmul(2,4)&mul[3,7]!^don't()_don't()mul(5,5)+mul(32,64](mul(11,8)undo()do()?mul(8,5))don't()mul(4,2)testd()mul(8,1)don't()mul(4,1)d()mul(8,1)";
        let result = part2(str.to_string());
        assert_eq!(result, "48");
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part1_input() {
        assert_eq!(
            part1(include_str!("../../.data/y2024/d3.txt").to_string()),
            "184122457"
        );
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part2_input() {
        assert_eq!(
            part2(include_str!("../../.data/y2024/d3.txt").to_string()),
            "107862689"
        );
    }
}
