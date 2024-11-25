use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Input {
    line: Vec<NumberOrSymbol>,
}

impl Input {
    pub fn create(line: Vec<NumberOrSymbol>) -> Input {
        Input { line }
    }
}

#[derive(Debug, Clone)]
pub struct Inputs {
    inputs: Vec<Input>,
}

impl Input {
    pub fn select(&mut self, symbol: Symbol) {
        for l in self.line.iter_mut() {
            if let NumberOrSymbol::Number(n) = l {
                let index = symbol.index as i32;
                if n.start_index - 2 < index && index < n.end_index + 2 {
                    n.selected = true;
                }
            }
        }
    }

    pub fn select_all(&mut self, other: &Input) {
        for l in other.line.iter() {
            if let NumberOrSymbol::Symbol(s) = l {
                self.select(s.clone());
            }
        }
    }

    pub fn find(&self, symbol: &mut Symbol) {
        for l in self.line.iter() {
            if let NumberOrSymbol::Number(n) = l {
                let index = symbol.index as i32;
                if n.start_index - 2 < index && index < n.end_index + 2 && symbol.char == '*' {
                    symbol.adjacent.push(n.value);
                }
            }
        }
    }

    pub fn find_adjacent(&mut self, previous: Option<&Input>, next: Option<&Input>) {
        let cloned = self.clone();
        for l in self.line.iter_mut() {
            if let NumberOrSymbol::Symbol(s) = l {
                cloned.find(s);
                if let Some(p) = previous {
                    p.find(s);
                }
                if let Some(p) = next {
                    p.find(s);
                }
            }
        }
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for l in self.line.iter() {
            match l {
                NumberOrSymbol::Number(n) => {
                    if n.selected {
                        s += &format!("({}) ", n.value);
                    } else {
                        s += &format!("{} ", n.value);
                    }
                }
                NumberOrSymbol::Symbol(i) => {
                    s += &format!(" {}({:?}) ", i.char, i.adjacent);
                }
            }
        }
        write!(f, "{}", s)
    }
}

impl Inputs {
    pub fn create(inputs: Vec<Input>) -> Inputs {
        Inputs { inputs }
    }

    pub fn sum(&mut self) -> (u32, u32) {
        let cloned = self.inputs.clone();

        for (i, input) in self.inputs.iter_mut().enumerate() {
            input.select_all(&input.clone());

            let previous = if i > 0 { cloned.get(i - 1) } else { None };
            if let Some(previous) = previous {
                input.select_all(previous);
            }
            if let Some(next) = cloned.get(i + 1) {
                input.select_all(next);
            }

            input.find_adjacent(previous, cloned.get(i + 1));
        }

        let mut sum = 0;
        let mut gears = 0;
        for input in self.inputs.iter() {
            for l in input.line.iter() {
                match l {
                    NumberOrSymbol::Number(n) if n.selected => sum += n.value,
                    NumberOrSymbol::Symbol(s) if s.adjacent.len() == 2 => {
                        gears += s.adjacent[0] * s.adjacent[1]
                    }
                    _ => {}
                }
            }
        }

        (sum, gears)
    }
}

#[derive(Debug, Clone)]
pub enum NumberOrSymbol {
    Number(Number),
    Symbol(Symbol),
}

#[derive(Debug, Clone)]
pub struct Number {
    value: u32,
    start_index: i32,
    end_index: i32,
    selected: bool,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    index: usize,
    char: char,
    adjacent: Vec<u32>,
}

pub fn part1(input: String) -> String {
    let lines: Vec<String> = input.split('\n').map(|s| s.to_string()).collect();
    let mut inputs: Vec<Input> = Vec::new();
    for line_str in lines {
        if line_str.is_empty() {
            continue;
        }
        let length = line_str.len();
        let mut num = 0;
        let mut start_index: i32 = -1;
        let mut end_index: i32;
        let mut line: Vec<NumberOrSymbol> = Vec::new();
        for i in 0..length {
            let c = line_str.chars().nth(i).unwrap();
            match c {
                c if char::is_digit(c, 10) => {
                    num = num * 10 + c.to_digit(10).unwrap();
                    if start_index == -1 {
                        start_index = i as i32;
                    }
                }
                '.' => {
                    if num != 0 {
                        end_index = (i - 1) as i32;
                        line.push(NumberOrSymbol::Number(Number {
                            value: num,
                            start_index,
                            end_index,
                            selected: false,
                        }));
                        num = 0;
                        start_index = -1;
                    }
                }
                c => {
                    if num != 0 {
                        end_index = (i - 1) as i32;
                        line.push(NumberOrSymbol::Number(Number {
                            value: num,
                            start_index,
                            end_index,
                            selected: false,
                        }));
                        num = 0;
                        start_index = -1;
                    }
                    line.push(NumberOrSymbol::Symbol(Symbol {
                        index: i,
                        char: c,
                        adjacent: Vec::new(),
                    }));
                }
            }
            if i == length - 1 && num != 0 {
                end_index = i as i32;
                line.push(NumberOrSymbol::Number(Number {
                    value: num,
                    start_index,
                    end_index,
                    selected: false,
                }));
            }
        }
        inputs.push(Input::create(line));
    }
    let mut inputs = Inputs::create(inputs);
    inputs.sum().0.to_string()
}

pub fn part2(input: String) -> String {
    part1(input)
}
