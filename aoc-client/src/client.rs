use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use reqwest::header::COOKIE;

/// Advent of Code client
///
/// This client is used to get input from the Advent of Code website and to submit solutions.
///
/// # Example
///
/// ```rust
/// use aoc_lib::Client;
///
/// fn example() {
///   let client = Client::new().unwrap();
///   
///   let input = client.get_input(2019, 1).unwrap();
///   let result = client.submit_solution(2019, 1, 1, "solution").unwrap();
///   println!("{}", result);
/// }
pub struct Client {
    session_token: String,
    client: reqwest::blocking::Client,
    cache_dir: std::path::PathBuf,
}

impl Client {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let file = std::env::current_exe()?;
        let mut base = file.parent().unwrap().parent().unwrap().parent().unwrap();
        if base.to_str().unwrap().ends_with("target") {
            base = base.parent().unwrap();
        }
        let mut folder_path = base.to_path_buf();
        folder_path.push("aoc-client/.session");

        println!("folder {:?}", folder_path);
        let session_token = std::fs::read_to_string(folder_path)
            .unwrap()
            .trim()
            .to_string();
        let mut cache_dir = base.to_path_buf();
        cache_dir.push(".data");

        fs::create_dir_all(&cache_dir).map_err(|err| {
            eprintln!(
                "Failed to create cache dir \"{}\": {}",
                cache_dir.display(),
                err
            );
            err
        })?;

        Ok(Self {
            cache_dir,
            session_token,
            client: reqwest::blocking::Client::new(),
        })
    }

    pub fn get_input(&self, year: u32, day: u32) -> Result<String, Box<dyn Error>> {
        if let Ok(input) = self.get_cached_input(year, day) {
            return Ok(input);
        }

        let input = self.download_input(year, day)?;
        self.cache_input(year, day, &input)?;

        Ok(input)
    }

    fn get_cached_input(&self, year: u32, day: u32) -> Result<String, Box<dyn Error>> {
        let path = self.cache_dir.join(format!("y{}/d{}.txt", year, day));
        Ok(std::fs::read_to_string(path)?)
    }

    fn cache_input(&self, year: u32, day: u32, input: &str) -> Result<(), Box<dyn Error>> {
        let path = self.cache_dir.join(format!("y{}/d{}.txt", year, day));
        // create the year folder if it doesn't exist
        fs::create_dir_all(path.parent().unwrap())?;

        let mut file = File::create(path)?;

        file.write_all(input.as_bytes())?;

        Ok(())
    }

    fn download_input(&self, year: u32, day: u32) -> Result<String, Box<dyn Error>> {
        let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
        let cookie = format!("session={}", self.session_token);
        let input = self
            .client
            .get(&url)
            .header(COOKIE, cookie)
            .send()?
            .error_for_status()?
            .text()?;

        Ok(input)
    }

    pub fn submit_solution(
        &self,
        year: u32,
        day: u32,
        part: u32,
        solution: &str,
    ) -> Result<String, Box<dyn Error>> {
        use select::document::Document;
        use select::predicate::Name;

        let url = format!("https://adventofcode.com/{}/day/{}/answer", year, day);
        let cookie = format!("session={}", self.session_token);

        let mut params = HashMap::new();
        params.insert("level", part.to_string());
        params.insert("answer", solution.into());

        let response = self
            .client
            .post(&url)
            .header(COOKIE, cookie)
            .form(&params)
            .send()?
            .error_for_status()?
            .text()?;

        let doc = Document::from(response.as_str());
        let node = doc.find(Name("main")).next().unwrap();
        let text = node.text();
        // let text = text.trim().split(".  ").next().unwrap_or("");
        let text = format!("{}.", text.trim());

        Ok(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_input() {
        let client = Client::new().unwrap();
        let result = client.get_input(2023, 6).unwrap();
        assert_eq!(
            result,
            "Time:        44     80     65     72\nDistance:   208   1581   1050   1102\n"
        );
    }

    #[test]
    fn test_input_is_cached() {
        let client = Client::new().unwrap();
        client.get_input(2023, 6).unwrap();
        let result = client.get_cached_input(2023, 6).unwrap();
        assert_eq!(
            result,
            "Time:        44     80     65     72\nDistance:   208   1581   1050   1102\n"
        );
    }

    #[test]
    fn test_caching() {
        let year = 1000;
        let day = 100;
        let client = Client::new().unwrap();
        client.cache_input(year, day, "test").unwrap();
        let res = client.get_cached_input(year, day).unwrap();
        assert_eq!(res, "test");
    }
}
