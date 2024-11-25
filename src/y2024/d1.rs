pub fn part1(_input: String) -> String {
    unimplemented!()
}

pub fn part2(_input: String) -> String {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = part1(str.to_string());
        assert_eq!(result, "142");
    }
}
