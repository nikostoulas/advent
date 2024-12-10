use std::collections::HashMap;
use std::collections::HashSet;

use parser::{Direction, MultiLineParser};

pub fn part1(input: String) -> String {
    let mut parser = parse_input(input);
    parser.advance_to("^");
    let mut cur = parser.point();
    let mut direction = Direction::Up;
    while parser.adnvance_to_with_direction(&'#', &direction) {
        direction = direction.next_4();
        let point = (parser.line(), parser.cursor());
        parser.fill(&'x', cur, point);
        cur = point;
    }
    let point = (parser.line(), parser.cursor());
    parser.fill(&'x', cur, point);

    parser.count_chars(&'x').to_string()
}

pub fn part2(input: String) -> String {
    let mut parser = parse_input(input);
    parser.advance_to("^");
    let start = parser.point();
    let mut direction = Direction::Up;
    let mut previous_point = start;
    let mut paths = vec![];
    while parser.adnvance_to_with_direction(&'#', &direction) {
        direction = direction.next_4();
        let point = (parser.line(), parser.cursor());
        paths.push((previous_point, point));
        parser.fill(&'x', previous_point, point);
        previous_point = point;
    }
    let point = (parser.line(), parser.cursor());
    paths.push((previous_point, point));
    let mut points = HashSet::new();
    for (from, to) in paths {
        let line_from = from.0.min(to.0);
        let line_to = from.0.max(to.0);
        let cursor_from = from.1.min(to.1);
        let cursor_to = from.1.max(to.1);

        for line in line_from..=line_to {
            for cursor in cursor_from..=cursor_to {
                if (line, cursor) == start {
                    continue;
                }
                let mut new_parser = parser.clone();
                new_parser.go_to((line, cursor)).set(&'#');
                new_parser.go_to(start);
                if does_obstacle_cause_cycle(&mut new_parser) {
                    points.insert((line, cursor));
                }
            }
        }
    }

    points.len().to_string()
}

fn does_obstacle_cause_cycle(parser: &mut MultiLineParser) -> bool {
    let mut previous_point = parser.point();
    let mut direction = Direction::Up;
    let mut points: HashMap<(usize, usize), usize> = HashMap::new();
    while parser.adnvance_to_with_direction(&'#', &direction) {
        direction = direction.next_4();
        let point = (parser.line(), parser.cursor());
        if parser.peek() == Some(&'z') {
            points.entry(point).and_modify(|e| *e += 1).or_insert(1);
            if points.get(&point).unwrap() > &1 {
                return true;
            }
        }
        parser.fill(&'z', previous_point, point);
        previous_point = point;
    }
    false
}

fn parse_input(input: String) -> MultiLineParser {
    MultiLineParser::new(&input)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part1() {
        let result = part1(INPUT.to_string());
        assert_eq!(result, "41");
        assert_eq!(
            part1(include_str!("../../.data/y2024/d6.txt").to_string()),
            "4515"
        );
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT.to_string());
        assert_eq!(result, "6");
    }

    #[test]
    #[ignore]
    fn test_part2_input() {
        assert_eq!(
            part2(include_str!("../../.data/y2024/d6.txt").to_string()),
            "1309"
        );
    }
}
