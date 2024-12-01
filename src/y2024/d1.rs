pub fn part1(input: String) -> String {
    let (mut first_list, mut second_list) = get_two_lists_from_input(input);
    first_list.sort();
    second_list.sort();
    let distance: i32 = first_list
        .iter()
        .zip(second_list.iter())
        .map(|(f, s)| (f - s).abs())
        .sum();
    distance.to_string()
}

pub fn part2(input: String) -> String {
    let (first_list, second_list) = get_two_lists_from_input(input);
    let mut sum = 0;
    for i in first_list.iter() {
        sum += i * second_list.iter().filter(|s| *s == i).count() as i32;
    }
    sum.to_string()
}

fn get_two_lists_from_input(input: String) -> (Vec<i32>, Vec<i32>) {
    let lines: Vec<String> = input.split('\n').map(|s| s.to_string()).collect();
    let lists: Vec<(i32, i32)> = lines
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut nums = s.split_whitespace().map(|s| s.parse::<i32>().unwrap());
            let first_num = nums.next().unwrap();
            let second_num = nums.next().unwrap();
            (first_num, second_num)
        })
        .collect();
    let first_list: Vec<i32> = lists.iter().map(|e| e.0).collect();
    let second_list: Vec<i32> = lists.iter().map(|e| e.1).collect();
    (first_list, second_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let str = "3   4
4   3
2   5
1   3
3   9
3   3
";
        let result = part1(str.to_string());
        assert_eq!(result, "11");
    }

    #[test]
    fn test_example_part2() {
        let str = "3   4
4   3
2   5
1   3
3   9
3   3
";
        let result = part2(str.to_string());
        assert_eq!(result, "31");
    }
}
