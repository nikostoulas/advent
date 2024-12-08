use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parser {
    characters: Vec<char>,
    cursor: usize,
}

impl Display for Parser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for char in &self.characters {
            write!(f, "{}", char)?;
        }
        Ok(())
    }
}

impl Parser {
    pub fn new(str: &str) -> Self {
        Self {
            characters: str.chars().collect(),
            cursor: 0,
        }
    }

    pub fn go_to(&mut self, to: usize) {
        self.cursor = to;
    }

    pub fn fill(&mut self, target: &char, from: usize, to: usize) {
        let cursor_from = from.min(to);
        let cursor_to = from.max(to).min(self.len() - 1);
        for cursor in cursor_from..=cursor_to {
            self.characters[cursor] = *target;
        }
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn peek(&self) -> Option<&char> {
        self.characters.get(self.cursor)
    }

    pub fn pop(&mut self) -> Option<&char> {
        let value = self.characters.get(self.cursor);
        if !self.is_done() {
            self.cursor += 1;
        }
        value
    }

    pub fn len(&self) -> usize {
        self.characters.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_done(&self) -> bool {
        self.cursor == self.characters.len()
    }

    pub fn advance_to(&mut self, target: &str) -> bool {
        let remaining = self.characters.iter().skip(self.cursor);
        let position = remaining.enumerate().position(|(j, c)| {
            c == &target.chars().next().unwrap()
                && target
                    .chars()
                    .enumerate()
                    .skip(1)
                    .all(|(i, t)| self.peek_at((j + i) as i32) == Some(&t))
        });
        if position.is_some() {
            self.cursor += position.unwrap() + target.len() - 1;
            true
        } else {
            self.cursor = self.characters.len();
            false
        }
    }

    pub fn peek_at(&self, num: i32) -> Option<&char> {
        if self.cursor as i32 + num < 0 {
            return None;
        }
        self.characters.get((self.cursor as i32 + num) as usize)
    }

    pub fn delete_between(&mut self, from: &str, to: &str) -> String {
        let mut position = self.cursor;
        let mut result = String::new();
        while !self.is_done() {
            self.advance_to(from);
            result.push_str(
                self.characters[position..self.cursor]
                    .iter()
                    .collect::<String>()
                    .as_str(),
            );
            self.advance_to(to);
            self.advance(1);
            position = self.cursor;
        }
        result
    }

    pub fn match_number(&mut self) -> Option<i64> {
        let mut number = String::new();
        while let Some(&c) = self.peek() {
            if char::is_digit(c, 10) {
                number.push(c);
                self.cursor += 1;
            } else {
                break;
            }
        }
        number.parse().ok()
    }

    pub fn match_number_up_to(&mut self, target: char) -> Option<i64> {
        let number = self.match_number();
        if self.peek() == Some(&target) {
            self.cursor += 1;
            return number;
        }
        None
    }

    pub fn split_to_numbers(&self, delimiter: &str) -> Vec<i64> {
        self.characters
            .iter()
            .skip(self.cursor)
            .collect::<String>()
            .split(delimiter)
            .map(|n| n.parse().unwrap())
            .collect()
    }

    pub fn advance(&mut self, num: usize) -> usize {
        if self.cursor + num > self.characters.len() {
            let remaining = num - (self.len() - self.cursor);
            self.cursor = self.characters.len();
            return remaining;
        }
        self.cursor += num;
        0
    }

    pub fn go_back(&mut self, num: usize) -> usize {
        if self.cursor < num {
            let remaining = num - self.cursor;
            self.cursor = 0;
            return remaining;
        }
        self.cursor -= num;
        0
    }

    pub fn reset(&mut self) {
        self.cursor = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peek() {
        let parser = Parser::new("hello");
        assert_eq!(parser.peek(), Some(&'h'));
    }

    #[test]
    fn test_advance() {
        let mut parser = Parser::new("hello");
        assert_eq!(parser.pop(), Some(&'h'));
        assert_eq!(parser.cursor(), 1);

        parser.advance(4);
        assert_eq!(parser.pop(), None);
        assert_eq!(parser.cursor(), 5);
    }

    #[test]
    fn test_advance_to() {
        let mut parser = Parser::new("hello");
        assert_eq!(parser.advance_to("ll"), true);
        assert_eq!(parser.cursor(), 3);
        assert_eq!(parser.advance_to("ll"), false);
    }

    #[test]
    fn test_advnace() {
        let mut parser = Parser::new("hello");
        parser.advance(3);
        assert_eq!(parser.cursor(), 3);
    }

    #[test]
    fn test_match_number() {
        let mut parser = Parser::new("asd123hello");
        parser.advance(3);
        assert_eq!(parser.match_number(), Some(123));
        assert_eq!(parser.cursor(), 6);
    }

    #[test]
    fn test_match_number_up_to() {
        let mut parser = Parser::new("asd123hello");
        parser.advance(3);
        assert_eq!(parser.match_number_up_to('h'), Some(123));
        assert_eq!(parser.cursor(), 7);
    }

    #[test]
    fn test_ignore_between() {
        let mut parser = Parser::new("hello[world]goodbye[world]");
        assert_eq!(parser.delete_between("[", "]"), "hellogoodbye");
    }
}
