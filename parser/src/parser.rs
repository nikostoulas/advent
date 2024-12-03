pub struct Parser {
    characters: Vec<char>,
    cursor: usize,
}

impl Parser {
    pub fn new(str: &str) -> Self {
        Self {
            characters: str.chars().collect(),
            cursor: 0,
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
        self.cursor += 1;
        value
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
                    .all(|(i, t)| self.peek_at(j + i) == Some(&t))
        });
        if position.is_some() {
            self.cursor += position.unwrap() + target.len();
            true
        } else {
            self.cursor = self.characters.len();
            false
        }
    }

    pub fn peek_at(&self, num: usize) -> Option<&char> {
        self.characters.get(self.cursor + num)
    }

    pub fn delete_between(&mut self, from: &str, to: &str) -> String {
        let mut position = self.cursor;
        let mut result = String::new();
        while !self.is_done() {
            self.advance_to(from);
            result.push_str(
                self.characters[position..self.cursor - 1]
                    .iter()
                    .collect::<String>()
                    .as_str(),
            );
            self.advance_to(to);
            position = self.cursor;
        }
        result
    }

    pub fn match_number(&mut self) -> Option<i32> {
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

    pub fn match_number_up_to(&mut self, target: char) -> Option<i32> {
        let number = self.match_number();
        if self.peek() == Some(&target) {
            self.cursor += 1;
            return number;
        }
        None
    }

    pub fn advance(&mut self, num: usize) {
        if self.cursor + num > self.characters.len() {
            panic!("Cannot advance past the end of the input");
        }
        self.cursor += num;
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
    }

    #[test]
    fn test_advance_to() {
        let mut parser = Parser::new("hello");
        assert_eq!(parser.advance_to("ll"), true);
        assert_eq!(parser.cursor(), 4);
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
