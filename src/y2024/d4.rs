use parser::MultiLineParser;
pub fn part1(input: String) -> String {
    let mut parser = parse_input(input);
    let mut sum = 0;
    while !parser.is_done() {
        let count = parser.word_count("XMAS").len();
        sum += count;
        parser.pop();
    }
    sum.to_string()
}

pub fn part2(input: String) -> String {
    let mut parser = parse_input(input);
    let mut sum = 0;
    while !parser.is_done() {
        if parser.diagonal_x_exists_in_any_order("MAS") {
            sum += 1;
        }
        parser.pop();
    }
    sum.to_string()
}

fn parse_input(input: String) -> MultiLineParser {
    MultiLineParser::new(&input)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn test_part1() {
        let result = part1(INPUT.to_string());
        assert_eq!(result, "18");
        assert_eq!(
            part1(include_str!("../../.data/y2024/d4.txt").to_string()),
            "2401"
        );
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT.to_string());
        assert_eq!(result, "9");
        assert_eq!(
            part2(include_str!("../../.data/y2024/d4.txt").to_string()),
            "1822"
        );
    }
}
