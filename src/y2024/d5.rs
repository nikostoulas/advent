use std::cmp::Ordering;

use parser::MultiLineParser;

pub fn part1(input: String) -> String {
    let (orders, pages) = parse_input(input);
    let sum: i64 = pages
        .iter()
        .filter(|page| is_ordered(page, &orders))
        .map(|page| page[page.len() / 2])
        .sum();

    sum.to_string()
}

pub fn part2(input: String) -> String {
    let (orders, pages) = parse_input(input);
    let sum: i64 = pages
        .iter()
        .filter(|page| !is_ordered(page, &orders))
        .map(|page| sort_using_order(page.to_vec(), &orders))
        .map(|page| page[page.len() / 2])
        .sum();

    sum.to_string()
}

fn parse_input(input: String) -> (Vec<Vec<i64>>, Vec<Vec<i64>>) {
    let (orders_str, pages_str) = input.split_once("\n\n").unwrap();
    let orders: Vec<Vec<i64>> = MultiLineParser::new(orders_str).split_to_numbers("|");
    let pages: Vec<Vec<i64>> = MultiLineParser::new(pages_str).split_to_numbers(",");
    (orders, pages)
}

fn is_ordered(page: &[i64], orders: &[Vec<i64>]) -> bool {
    orders.iter().all(|order| {
        let position_a = page.iter().position(|p| p == &order[0]);
        let position_b = page.iter().position(|p| p == &order[1]);
        !matches!((position_a, position_b), (Some(a), Some(b)) if a > b)
    })
}

fn sort_using_order(mut page: Vec<i64>, orders: &[Vec<i64>]) -> Vec<i64> {
    page.sort_by(|a, b| {
        if orders.iter().any(|order| order == &[*b, *a]) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    page
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part1() {
        let result = part1(INPUT.to_string());
        assert_eq!(result, "143");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT.to_string());
        assert_eq!(result, "123");
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part1_input() {
        assert_eq!(
            part1(include_str!("../../.data/y2024/d5.txt").to_string()),
            "4872"
        );
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part2_input() {
        assert_eq!(
            part2(include_str!("../../.data/y2024/d5.txt").to_string()),
            "5564"
        );
    }
}
