#[allow(unused_imports)]
use std::{error::Error, fs};

pub struct Input {
    a: String,
}

pub enum Number {
    One,
    Two,
    Threw,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    NAN,
}

impl Number {
    pub fn create(a: String) -> Number {
        if a.ends_with("one") || a.starts_with("one") {
            return Number::One;
        }
        if a.ends_with("two") || a.starts_with("two") {
            return Number::Two;
        }
        if a.ends_with("three") || a.starts_with("three") {
            return Number::Threw;
        }
        if a.ends_with("four") || a.starts_with("four") {
            return Number::Four;
        }
        if a.ends_with("five") || a.starts_with("five") {
            return Number::Five;
        }
        if a.ends_with("six") || a.starts_with("six") {
            return Number::Six;
        }
        if a.ends_with("seven") || a.starts_with("seven") {
            return Number::Seven;
        }
        if a.ends_with("eight") || a.starts_with("eight") {
            return Number::Eight;
        }
        if a.ends_with("nine") || a.starts_with("nine") {
            return Number::Nine;
        }
        Number::NAN
    }

    pub fn number(&self) -> u32 {
        match self {
            Number::One => 1,
            Number::Two => 2,
            Number::Threw => 3,
            Number::Four => 4,
            Number::Five => 5,
            Number::Six => 6,
            Number::Seven => 7,
            Number::Eight => 8,
            Number::Nine => 9,
            Number::NAN => 0,
        }
    }
}

impl Input {
    pub fn create(a: String) -> Input {
        Input { a }
    }

    pub fn number(&self) -> u32 {
        let mut num = 0;
        let length = self.a.len();
        for c in 0..length {
            let a = self.a.chars().nth(c).unwrap();
            if a.is_digit(10) {
                num = num * 10 + a.to_digit(10).unwrap();
                break;
            }
            let snum: Number = Number::create(self.a[..c + 1].to_string());
            if snum.number() != 0 {
                num = num * 10 + snum.number();
                break;
            }
        }
        for c in (0..length).rev() {
            let a = self.a.chars().nth(c).unwrap();
            if a.is_digit(10) {
                num = num * 10 + a.to_digit(10).unwrap();
                break;
            }
            let snum: Number = Number::create(self.a[c - 1..].to_string());
            if snum.number() != 0 {
                num = num * 10 + snum.number();
                break;
            }
        }
        num
    }
}

pub fn part1(input: String) -> String {
    let lines: Vec<String> = input.split('\n').map(|s| s.to_string()).collect();
    let mut sum = 0;
    for line in lines.iter() {
        sum += Input::create(line.to_string()).number();
    }
    sum.to_string()
}

pub fn part2(input: String) -> String {
    let lines: Vec<String> = input.split('\n').map(|s| s.to_string()).collect();
    let mut sum = 0;
    for line in lines.iter() {
        sum += Input::create(line.to_string()).number();
    }
    sum.to_string()
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
