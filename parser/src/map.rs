use crate::MultiLineParser;
use std::collections::HashMap;
pub type Point = (usize, usize);
pub type Map = HashMap<char, Vec<Point>>;

impl From<&mut MultiLineParser> for Map {
    fn from(parser: &mut MultiLineParser) -> Self {
        let mut map: Map = HashMap::new();

        for (char, point) in parser.iter() {
            map.entry(char).or_insert(vec![]).push(point);
        }
        parser.reset();
        map
    }
}
