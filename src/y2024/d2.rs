pub fn part1(input: String) -> String {
    let lists = parse_input(input);
    let safe_list_count = lists
        .iter()
        .filter(|list| is_increasing(list) || is_decreasing(list))
        .count();
    safe_list_count.to_string()
}

pub fn part2(input: String) -> String {
    let lists = parse_input(input);
    let safe_list_count = lists
        .iter()
        .filter(|list| is_increasing_with_one_skipped(list) || is_decreasing_with_one_skipped(list))
        .count();
    safe_list_count.to_string()
}

fn parse_input(input: String) -> Vec<Vec<i32>> {
    let lines: Vec<String> = input.split('\n').map(|s| s.to_string()).collect();
    let lists: Vec<Vec<i32>> = lines
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| {
            let nums = s
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            nums
        })
        .collect();
    lists
}

fn is_increasing(list: &[i32]) -> bool {
    list.windows(2)
        .all(|l| l[1] - l[0] >= 1 && l[1] - l[0] <= 3)
}

fn is_decreasing(list: &[i32]) -> bool {
    list.windows(2)
        .all(|l| l[0] - l[1] >= 1 && l[0] - l[1] <= 3)
}

fn is_increasing_with_one_skipped(list: &[i32]) -> bool {
    for (j, i) in list.windows(2).enumerate() {
        let increasing = i[1] - i[0] >= 1 && i[1] - i[0] <= 3;
        if !increasing {
            return is_increasing(&list_without_element_at(list, j))
                || is_increasing(&list_without_element_at(list, j + 1));
        }
    }
    true
}

fn is_decreasing_with_one_skipped(list: &[i32]) -> bool {
    for (j, i) in list.windows(2).enumerate() {
        let decreasing = i[0] - i[1] >= 1 && i[0] - i[1] <= 3;
        if !decreasing {
            return is_decreasing(&list_without_element_at(list, j))
                || is_decreasing(&list_without_element_at(list, j + 1));
        }
    }
    true
}

fn list_without_element_at(list: &[i32], index: usize) -> Vec<i32> {
    let mut list: Vec<i32> = list.to_vec();
    list.remove(index);
    list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let result = part1(str.to_string());
        assert_eq!(result, "2");
    }

    #[test]
    fn test_example_part2() {
        let str = "7 6 4 5 3
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
        let result = part2(str.to_string());
        assert_eq!(result, "5");
    }
}
