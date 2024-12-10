use crate::Direction;
use crate::Parser;
use crate::Point;
use std::fmt::Display;

use Direction::{Down, DownLeft, Left, LeftUp, Right, RightDown, Up, UpRight};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiLineParser {
    parsers: Vec<Parser>,
    line: usize,
}

impl Display for MultiLineParser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for parser in &self.parsers {
            writeln!(f, "{}", parser)?;
        }
        Ok(())
    }
}

pub struct MultiLineParserIterator<'a> {
    parser: &'a mut MultiLineParser,
}

impl<'a> Iterator for MultiLineParserIterator<'a> {
    type Item = (char, Point);

    fn next(&mut self) -> Option<Self::Item> {
        if self.parser.is_done() {
            return None;
        }
        let point = self.parser.point();
        Some((*self.parser.pop()?, point))
    }
}

impl MultiLineParser {
    pub fn new(str: &str) -> Self {
        let parsers = str
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(Parser::new)
            .collect();

        MultiLineParser { parsers, line: 0 }
    }

    pub fn iter(&mut self) -> MultiLineParserIterator {
        MultiLineParserIterator { parser: self }
    }

    pub fn len(&self) -> usize {
        self.parsers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn point(&self) -> Point {
        (self.line(), self.cursor())
    }

    pub fn cursor_len(&self) -> usize {
        self.parsers[0].len()
    }

    pub fn match_number(&mut self) -> Vec<Option<i64>> {
        self.parsers.iter_mut().map(|p| p.match_number()).collect()
    }

    pub fn match_number_up_to(&mut self, target: char) -> Vec<Option<i64>> {
        self.parsers
            .iter_mut()
            .map(|p| p.match_number_up_to(target))
            .collect()
    }

    pub fn advance_all_lines(&mut self, num: usize) {
        self.parsers.iter_mut().for_each(|p| {
            p.advance(num);
        });
    }

    pub fn reset(&mut self) {
        self.line = 0;
        self.parsers.iter_mut().for_each(|p| p.reset());
    }

    pub fn split_to_numbers(&self, delimiter: &str) -> Vec<Vec<i64>> {
        self.parsers
            .iter()
            .map(|p| p.split_to_numbers(delimiter))
            .collect()
    }

    pub fn peek(&self) -> Option<&char> {
        let parser = self.parsers.get(self.line);
        if let Some(parser) = parser {
            parser.peek()
        } else {
            None
        }
    }

    pub fn peek_at(&self, line: i32, cursor: i32) -> Option<&char> {
        if self.line as i32 + line < 0 {
            return None;
        }
        let parser = self.parsers.get((self.line as i32 + line) as usize);
        let cur_cursor = self.cursor();
        if let Some(parser) = parser {
            parser.peek_at(cur_cursor as i32 + cursor - parser.cursor() as i32)
        } else {
            None
        }
    }

    pub fn advance(&mut self, mut num: usize) {
        while num > 0 && !self.is_done() {
            num = self.parsers[self.line].advance(num);
            if self.parsers[self.line].is_done() {
                self.line += 1;
            }
        }
    }

    pub fn advance_to(&mut self, target: &str) -> bool {
        while !self.parsers[self.line].advance_to(target) && !self.is_done() {
            self.line += 1;
        }

        self.is_done()
    }

    pub fn go_to(&mut self, to: (usize, usize)) -> &mut Self {
        self.line = to.0;
        if self.line >= self.parsers.len() {
            self.line = self.parsers.len();
            return self;
        }
        self.parsers[self.line].go_to(to.1);
        self
    }

    pub fn go_to_symmetrically(&mut self, mut to: (i32, i32)) -> &mut Self {
        to.0 %= self.parsers.len() as i32;
        if to.0 < 0 {
            to.0 += self.parsers.len() as i32;
        }
        self.line = to.0 as usize;
        self.parsers[self.line].go_to_symmetrically(to.1);
        self
    }

    pub fn fill(&mut self, target: &char, from: (usize, usize), to: (usize, usize)) {
        let line_from = from.0.min(to.0);
        let line_to = from.0.max(to.0).min(self.parsers.len() - 1);
        for line in line_from..=line_to {
            self.parsers[line].fill(target, from.1, to.1);
        }
    }

    pub fn set(&mut self, target: &char) {
        self.parsers.get_mut(self.line).map(|p| p.set(target));
    }

    pub fn count_chars(&mut self, target: &char) -> usize {
        self.reset();
        let mut count = 0;
        while !self.is_done() {
            if self.pop() == Some(target) {
                count += 1;
            }
        }
        count
    }

    pub fn adnvance_to_with_direction(&mut self, target: &char, direction: &Direction) -> bool {
        let mut i: i32 = 1;
        loop {
            let value = match direction {
                Right => self.peek_at(0, i),
                RightDown => self.peek_at(i, i),
                Down => self.peek_at(i, 0),
                DownLeft => self.peek_at(i, -i),
                Left => self.peek_at(0, -i),
                LeftUp => self.peek_at(-i, -i),
                Up => self.peek_at(-i, 0),
                UpRight => self.peek_at(-i, i),
            };
            if value == Some(target) || value.is_none() {
                let i = (i - 1) as usize;
                let ret = value.is_some();
                let cursor = self.cursor();
                match direction {
                    Right => {
                        self.parsers[self.line].go_to(cursor + i);
                    }
                    RightDown => {
                        self.line += i;
                        self.parsers[self.line].go_to(cursor + i);
                    }
                    Down => {
                        self.line += i;
                        self.parsers[self.line].go_to(cursor);
                    }
                    DownLeft => {
                        self.line += i;
                        self.parsers[self.line].go_to(cursor - i);
                    }
                    Left => {
                        self.parsers[self.line].go_to(cursor - i);
                    }
                    LeftUp => {
                        self.line -= i;
                        self.parsers[self.line].go_to(cursor - i);
                    }
                    Up => {
                        self.line -= i;
                        self.parsers[self.line].go_to(cursor);
                    }
                    UpRight => {
                        self.line -= i;
                        self.parsers[self.line].go_to(cursor + i);
                    }
                };

                return ret;
            }
            i += 1;
        }
    }

    pub fn pop(&mut self) -> Option<&char> {
        let line_change;
        {
            let cursor = self.cursor();
            let len = self.parser()?.len();
            if self.is_done() {
                return None;
            }
            line_change = cursor == len - 1;
        }

        let parser = self.parsers.get_mut(self.line);
        let value = parser?.pop();
        if line_change {
            self.line += 1;
        }
        value
    }

    pub fn is_done(&self) -> bool {
        self.line == self.parsers.len()
    }

    fn parser(&self) -> Option<&Parser> {
        self.parsers.get(self.line)
    }

    pub fn cursor(&self) -> usize {
        if self.is_done() {
            return self.parsers[self.parsers.len() - 1].cursor();
        }
        self.parsers[self.line].cursor()
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn peek_with_direction(&self, num: usize, direction: &Direction) -> Option<String> {
        let mut str: String = String::new();
        for i in 0..num {
            let i = i as i32;
            let value = match direction {
                Right => self.peek_at(0, i),
                RightDown => self.peek_at(i, i),
                Down => self.peek_at(i, 0),
                DownLeft => self.peek_at(i, -i),
                Left => self.peek_at(0, -i),
                LeftUp => self.peek_at(-i, -i),
                Up => self.peek_at(-i, 0),
                UpRight => self.peek_at(-i, i),
            };
            str.push(*value?);
        }
        Some(str)
    }

    pub fn peek_next_with_direction(&self, direction: &Direction) -> Option<&char> {
        let i = 1;
        let value = match direction {
            Right => self.peek_at(0, i),
            RightDown => self.peek_at(i, i),
            Down => self.peek_at(i, 0),
            DownLeft => self.peek_at(i, -i),
            Left => self.peek_at(0, -i),
            LeftUp => self.peek_at(-i, -i),
            Up => self.peek_at(-i, 0),
            UpRight => self.peek_at(-i, i),
        };
        value
    }

    pub fn advance_with_direction(&mut self, num: usize, direction: &Direction) {
        let cursor = self.cursor();
        match direction {
            Right => {
                self.parsers[self.line].go_to(cursor + num);
            }
            RightDown => {
                self.line += num;
                self.parsers[self.line].go_to(cursor + num);
            }
            Down => {
                self.line += num;
                self.parsers[self.line].go_to(cursor);
            }
            DownLeft => {
                self.line += num;
                self.parsers[self.line].go_to(cursor - num);
            }
            Left => {
                self.parsers[self.line].go_to(cursor - num);
            }
            LeftUp => {
                self.line -= num;
                self.parsers[self.line].go_to(cursor - num);
            }
            Up => {
                self.line -= num;
                self.parsers[self.line].go_to(cursor);
            }
            UpRight => {
                self.line -= num;
                self.parsers[self.line].go_to(cursor + num);
            }
        };
    }

    pub fn word_count(&self, word: &str) -> Vec<&Direction> {
        let length = word.len();
        let mut result = vec![];
        for dir in Direction::VALUES_8.iter() {
            let peeked = self.peek_with_direction(length, dir);
            if let Some(peeked) = peeked {
                if peeked == word {
                    result.push(dir);
                }
            }
        }
        result
    }

    pub fn diagonal_x_exists(&mut self, words: Vec<&str>) -> bool {
        let length = words[0].len();
        if words.iter().any(|w| w.len() != length) {
            return false;
        }
        let peeked = self.peek_with_direction(length, &RightDown);
        if let Some(peeked) = peeked {
            if words.iter().any(|w| peeked == **w) {
                self.parsers[self.line].advance(length - 1);
                let peeked = self.peek_with_direction(length, &DownLeft);
                self.parsers[self.line].go_back(length - 1);
                words.iter().any(|w| peeked == Some(w.to_string()))
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn diagonal_x_exists_in_any_order(&mut self, word: &str) -> bool {
        let reverse = word.chars().rev().collect::<String>();
        self.diagonal_x_exists(vec![word, &reverse])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peek() {
        let parser = MultiLineParser::new("hello\nworld");
        assert_eq!(parser.peek(), Some(&'h'));
    }

    #[test]
    fn test_peek_at() {
        let parser = MultiLineParser::new("he\nllo");
        assert_eq!(parser.peek_at(0, 0), Some(&'h'));
        assert_eq!(parser.peek_at(0, 1), Some(&'e'));
        assert_eq!(parser.peek_at(0, 2), None);
        assert_eq!(parser.peek_at(1, 0), Some(&'l'));
        assert_eq!(parser.peek_at(1, 1), Some(&'l'));
        assert_eq!(parser.peek_at(1, 2), Some(&'o'));
        assert_eq!(parser.peek_at(1, 3), None);
        assert_eq!(parser.peek_at(2, 0), None);
    }

    #[test]
    fn test_peek_at_when_advanced() {
        let mut parser = MultiLineParser::new("he\nllo");
        parser.advance(3);
        assert_eq!(parser.peek_at(0, 0), Some(&'l'));
        assert_eq!(parser.peek_at(0, -1), Some(&'l'));
        assert_eq!(parser.peek_at(-1, 0), Some(&'e'));
        assert_eq!(parser.peek_at(-1, -1), Some(&'h'));
    }

    #[test]
    fn test_pop() {
        let mut parser = MultiLineParser::new("he\nllo");
        assert_eq!(parser.cursor(), 0);
        assert_eq!(parser.pop(), Some(&'h'));
        assert_eq!(parser.cursor(), 1);
        assert_eq!(parser.pop(), Some(&'e'));
        assert_eq!(parser.cursor(), 0);
        assert_eq!(parser.pop(), Some(&'l'));
        assert_eq!(parser.cursor(), 1);
        assert_eq!(parser.pop(), Some(&'l'));
        assert_eq!(parser.cursor(), 2);
        assert_eq!(parser.pop(), Some(&'o'));
        assert_eq!(parser.cursor(), 3);
        assert_eq!(parser.pop(), None);
        assert_eq!(parser.cursor(), 3);
    }

    #[test]
    fn test_advance() {
        let mut parser = MultiLineParser::new("hello\nworld");
        parser.advance(5);
        assert_eq!(parser.peek(), Some(&'w'));
    }

    #[test]
    fn test_advance_to() {
        let mut parser = MultiLineParser::new("hello\nworld");
        parser.advance_to("w");
        assert_eq!(parser.peek(), Some(&'w'));
    }

    #[test]
    fn test_is_done() {
        let mut parser = MultiLineParser::new("hello\nworld");
        parser.advance(15);
        assert_eq!(parser.is_done(), true);
    }

    #[test]
    fn test_peek_with_direction() {
        let mut parser = MultiLineParser::new("hello\nworld");
        assert_eq!(parser.peek_with_direction(2, &Down), Some("hw".to_string()));
        assert_eq!(
            parser.peek_with_direction(2, &RightDown),
            Some("ho".to_string())
        );
        assert_eq!(parser.peek_with_direction(2, &Up), None);
        assert_eq!(
            parser.peek_with_direction(5, &Right),
            Some("hello".to_string())
        );
        parser.advance(4);
        assert_eq!(
            parser.peek_with_direction(2, &DownLeft),
            Some("ol".to_string())
        );
        assert_eq!(parser.peek_with_direction(2, &Left), Some("ol".to_string()));
        parser.line = 1;
        assert_eq!(
            parser.peek_with_direction(2, &Right),
            Some("wo".to_string())
        );
        parser.advance(4);
        assert_eq!(
            parser.peek_with_direction(2, &LeftUp),
            Some("dl".to_string())
        );
        assert_eq!(parser.peek_with_direction(2, &Up), Some("do".to_string()));
        parser.parsers[1].go_back(1);
        assert_eq!(
            parser.peek_with_direction(2, &UpRight),
            Some("lo".to_string())
        );
    }

    #[test]
    fn test_word_count() {
        let parser = MultiLineParser::new("hello\nworld");
        assert_eq!(parser.word_count("hello"), vec![&Right]);
        assert_eq!(parser.word_count("hw"), vec![&Down]);
    }

    #[test]
    fn test_diagonal_x_exists() {
        let mut parser = MultiLineParser::new("hello\nworld");
        assert_eq!(parser.diagonal_x_exists(vec!["ho", "ew"]), true);
        assert_eq!(parser.diagonal_x_exists(vec!["ho", "wo"]), false);
    }

    #[test]
    fn test_go_to_symmetrically() {
        let mut parser = MultiLineParser::new("hello\nworld\n12345");
        parser.go_to_symmetrically((-1, -1));
        assert_eq!(parser.peek(), Some(&'5'));
        parser.go_to_symmetrically((0, 5));
        assert_eq!(parser.peek(), Some(&'h'));
        parser.go_to_symmetrically((3, 5));
        assert_eq!(parser.peek(), Some(&'h'));
        parser.go_to_symmetrically((-2, -2));
        assert_eq!(parser.peek(), Some(&'l'));
    }
}
