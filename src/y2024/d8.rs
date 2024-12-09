use parser::{Map, MultiLineParser};

pub fn part1(input: String) -> String {
    let mut parser = parse_input(input);
    let mut map: Map = (&mut parser).into();
    map.remove(&'.');
    let mut antinodes = get_antinodes_from_map(map, parser);
    antinodes.count_chars(&'#').to_string()
}

pub fn part2(input: String) -> String {
    let mut parser = parse_input(input);
    let mut map: Map = (&mut parser).into();
    map.remove(&'.');
    let mut antinodes = get_antinodes_from_map_part2(map, parser);
    antinodes.count_chars(&'#').to_string()
}

fn parse_input(input: String) -> MultiLineParser {
    MultiLineParser::new(&input)
}

fn get_antinodes_from_map(map: Map, parser: MultiLineParser) -> MultiLineParser {
    let mut new_parser = parser.clone();
    for (_, points) in map.into_iter() {
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                if 2 * points[i].0 >= points[j].0 && 2 * points[i].1 >= points[j].1 {
                    let antinode = (2 * points[i].0 - points[j].0, 2 * points[i].1 - points[j].1);
                    new_parser.go_to(antinode).set(&'#');
                }
                if 2 * points[j].0 >= points[i].0 && 2 * points[j].1 >= points[i].1 {
                    let antinode = (2 * points[j].0 - points[i].0, 2 * points[j].1 - points[i].1);
                    new_parser.go_to(antinode).set(&'#');
                }
            }
        }
    }
    new_parser
}

fn get_antinodes_from_map_part2(map: Map, parser: MultiLineParser) -> MultiLineParser {
    let mut new_parser = parser.clone();
    for (_, points) in map.into_iter() {
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                let diff_line = points[j].0 - points[i].0;
                let diff_cursor = points[i].1.max(points[j].1) - points[i].1.min(points[j].1);
                let mut start = points[i];
                if points[i].1 > points[j].1 {
                    while start.0 >= diff_line && start.1 + diff_cursor < new_parser.cursor_len() {
                        start.0 -= diff_line;
                        start.1 += diff_cursor;
                    }
                    new_parser.go_to(start).set(&'#');
                    while start.0 < new_parser.len() && start.1 >= diff_cursor {
                        start.0 += diff_line;
                        start.1 -= diff_cursor;
                        new_parser.go_to(start).set(&'#');
                    }
                } else {
                    while start.0 >= diff_line && start.1 >= diff_cursor {
                        start.0 -= diff_line;
                        start.1 -= diff_cursor;
                    }
                    while start.0 < new_parser.len() && start.1 < new_parser.cursor_len() {
                        new_parser.go_to(start).set(&'#');
                        start.0 += diff_line;
                        start.1 += diff_cursor;
                    }
                }
            }
        }
    }
    new_parser
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    fn test_part1() {
        let result = part1(INPUT.to_string());
        assert_eq!(result, "14");
        assert_eq!(
            part1(include_str!("../../.data/y2024/d8.txt").to_string()),
            "220"
        );
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT.to_string());
        assert_eq!(result, "34");
        assert_eq!(
            part2(include_str!("../../.data/y2024/d8.txt").to_string()),
            "813"
        );
    }
}
