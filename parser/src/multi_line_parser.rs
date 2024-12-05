#[derive(Debug)]
pub struct MultiLineParser {
    lines: Vec<String>,
    characters: Vec<Vec<char>>,
    line: usize,
    cursor: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Right,
    RightDown,
    Down,
    DownLeft,
    Left,
    LeftUp,
    Up,
    UpRight,
}
use Direction::{Down, DownLeft, Left, LeftUp, Right, RightDown, Up, UpRight};

impl Direction {
    const VALUES: [Self; 8] = [Right, RightDown, Down, DownLeft, Left, LeftUp, Up, UpRight];
}

impl MultiLineParser {
    pub fn new(str: &str) -> Self {
        let lines: Vec<String> = str
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        MultiLineParser {
            characters: lines
                .iter()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect(),
            lines,
            line: 0,
            cursor: 0,
        }
    }

    pub fn split_to_numbers(&self, delimiter: &str) -> Vec<Vec<i32>> {
        self.lines
            .iter()
            .map(|line| line.split(delimiter).map(|n| n.parse().unwrap()).collect())
            .collect()
    }

    pub fn peek(&self) -> Option<&char> {
        let line = self.characters.get(self.line);
        if let Some(line) = line {
            line.get(self.cursor)
        } else {
            None
        }
    }

    pub fn peek_at(&self, line: i32, cursor: i32) -> Option<&char> {
        if self.line as i32 + line < 0 || self.cursor as i32 + cursor < 0 {
            return None;
        }
        let line = self.characters.get((self.line as i32 + line) as usize);
        if let Some(line) = line {
            line.get((self.cursor as i32 + cursor) as usize)
        } else {
            None
        }
    }

    pub fn advance(&mut self, num: usize) {
        self.cursor += num;
        if self.cursor >= self.characters[self.line].len() {
            let remainder = self.cursor % self.characters[self.line].len();
            self.line += self.cursor / self.characters[self.line].len();
            self.cursor = remainder;
        }
        if self.line > self.characters.len() {
            self.line = self.characters.len();
        }
        if self.is_done() {
            self.cursor = 0;
        }
    }

    pub fn pop(&mut self) -> Option<&char> {
        let line = self.characters.get(self.line);
        if let Some(line) = line {
            let value = line.get(self.cursor);
            if self.is_done() {
                return value;
            }
            if self.cursor == line.len() - 1 {
                self.cursor = 0;
                self.line += 1;
            } else {
                self.cursor += 1;
            }
            return value;
        }
        None
    }

    pub fn is_done(&self) -> bool {
        self.line == self.characters.len()
    }

    pub fn cursor(&self) -> usize {
        self.cursor
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
            if let Some(value) = value {
                str.push(*value);
            } else {
                return None;
            }
        }
        Some(str)
    }

    pub fn word_count(&self, word: &str) -> Vec<&Direction> {
        let length = word.len();
        let mut result = vec![];
        for dir in Direction::VALUES.iter() {
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
                self.cursor += length - 1;
                let peeked = self.peek_with_direction(length, &DownLeft);
                self.cursor -= length - 1;
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
        let parser = MultiLineParser::new("hello\nworld");
        assert_eq!(parser.peek_at(1, 1), Some(&'o'));
    }

    #[test]
    fn test_pop() {
        let mut parser = MultiLineParser::new("he\nllo");
        assert_eq!(parser.pop(), Some(&'h'));
        assert_eq!(parser.pop(), Some(&'e'));
        assert_eq!(parser.pop(), Some(&'l'));
        assert_eq!(parser.pop(), Some(&'l'));
        assert_eq!(parser.pop(), Some(&'o'));
        assert_eq!(parser.pop(), None);
    }

    #[test]
    fn test_advance() {
        let mut parser = MultiLineParser::new("hello\nworld");
        parser.advance(5);
        assert_eq!(parser.peek(), Some(&'w'));
    }

    #[test]
    fn test_is_done() {
        let mut parser = MultiLineParser::new("hello\nworld");
        parser.advance(15);
        println!("{:?}", parser);
        assert_eq!(parser.is_done(), true);
    }

    #[test]
    fn test_peek_with_direction() {
        let parser = MultiLineParser::new("hello\nworld");
        assert_eq!(parser.peek_with_direction(2, &Down), Some("hw".to_string()));
        assert_eq!(parser.peek_with_direction(2, &Up), None);
        assert_eq!(
            parser.peek_with_direction(5, &Right),
            Some("hello".to_string())
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
}
