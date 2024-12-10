use parser::{Direction, Map, MultiLineParser, Point};
use std::collections::HashSet;

pub fn part1(input: String) -> String {
    let (mut parser, map) = parse_input(&input);
    let starts = map.get(&'0').unwrap();

    let sum = starts
        .iter()
        .map(|p| dfs(&mut parser, p, '9'))
        .sum::<usize>();
    sum.to_string()
}

pub fn part2(input: String) -> String {
    let (mut parser, map) = parse_input(&input);
    let starts = map.get(&'0').unwrap();

    let sum = starts
        .iter()
        .map(|p| dfs_part2(&mut parser, p, '9'))
        .sum::<usize>();
    sum.to_string()
}

fn parse_input(input: &str) -> (MultiLineParser, Map) {
    let mut parser = MultiLineParser::new(input);
    let map: Map = (&mut parser).into();
    (parser, map)
}

fn dfs(parser: &mut MultiLineParser, source: &Point, needle: char) -> usize {
    parser.go_to(*source);
    let points = walk(parser, needle);
    let set: HashSet<&Point> = points.iter().collect();
    set.len()
}

fn dfs_part2(parser: &mut MultiLineParser, source: &Point, needle: char) -> usize {
    parser.go_to(*source);
    let points = walk(parser, needle);
    points.len()
}

fn walk(parser: &mut MultiLineParser, needle: char) -> Vec<Point> {
    if parser.peek() == Some(&needle) {
        return vec![parser.point()];
    }
    let mut points = vec![];
    for direction in Direction::VALUES_4 {
        let next = parser
            .peek_next_with_direction(&direction)
            .unwrap_or(&'0')
            .to_string();
        let cur = parser.peek().unwrap_or(&'0').to_string();

        if next.parse::<usize>().unwrap() == cur.parse::<usize>().unwrap() + 1 {
            parser.advance_with_direction(1, &direction);
            points.append(&mut walk(parser, needle));
            parser.advance_with_direction(1, &direction.opposite());
        }
    }

    points
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part1() {
        let result = part1(INPUT.to_string());
        assert_eq!(result, "36");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT.to_string());
        assert_eq!(result, "81");
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part1_input() {
        assert_eq!(
            part1(include_str!("../../.data/y2024/d10.txt").to_string()),
            "820"
        );
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part2_input() {
        assert_eq!(
            part2(include_str!("../../.data/y2024/d10.txt").to_string()),
            "1786"
        );
    }
}
