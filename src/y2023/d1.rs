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

pub struct FileInput {
    lines: Vec<String>,
}

impl FileInput {
    pub async fn create() -> Result<FileInput, Box<dyn Error>> {
        let resp: String =
            fs::read_to_string("./src/input.txt").expect("Should have been able to read the file");
        let lines: Vec<String> = resp.split('\n').map(|s| s.to_string()).collect();
        Ok(FileInput { lines })
    }

    pub fn sum(&self) -> u32 {
        let mut sum = 0;
        for line in self.lines.iter() {
            sum += Input::create(line.to_string()).number();
        }
        sum
    }
}

pub fn part1(input: String) -> String {
    let input = Input::create(input);
    input.number().to_string()
}

pub fn part2(input: String) -> String {
    let input = Input::create(input);
    input.number().to_string()
}
