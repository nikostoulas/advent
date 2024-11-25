use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;

use reqwest::header::COOKIE;

use crate::SolutionDay;

type ClientError = Box<dyn Error>;
type Params<'a> = HashMap<&'a str, String>;

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
    pub fn new() -> Result<Self, ClientError> {
        Ok(Self {
            cache_dir: Self::cache_dir()?,
            session_token: Self::session_token()?,
            client: reqwest::blocking::Client::new(),
        })
    }

    pub fn get_input(&self, solution_day: &SolutionDay) -> Result<String, ClientError> {
        if let Ok(input) = self.get_cached_input(solution_day) {
            return Ok(input);
        }
        let input = self.download_input(solution_day)?;
        self.cache_input(solution_day, &input)?;
        Ok(input)
    }

    pub fn submit_solution(
        &self,
        solution_day: &SolutionDay,
        solution: &str,
    ) -> Result<String, ClientError> {
        use select::document::Document;
        use select::predicate::Name;
        let SolutionDay { year, day, part } = solution_day;

        let url = format!("https://adventofcode.com/{}/day/{}/answer", year, day);
        let mut params = Params::new();
        params.insert("level", part.to_string());
        params.insert("answer", solution.into());

        let response = self.post_request(&url, &params)?;

        let doc = Document::from(response.as_str());
        let node = doc.find(Name("main")).next().unwrap();
        let text = node.text();
        let text = format!("{}.", text.trim());
        Ok(text)
    }

    fn cache_dir() -> Result<std::path::PathBuf, ClientError> {
        let cache_dir = Self::root_folder()?.join(".data");
        fs::create_dir_all(&cache_dir)?;
        Ok(cache_dir)
    }

    fn session_token() -> Result<String, ClientError> {
        let folder_path = Self::root_folder()?.join("aoc-client/.session");
        let token = std::fs::read_to_string(folder_path)?.trim().to_string();
        Ok(token)
    }

    fn root_folder() -> Result<std::path::PathBuf, ClientError> {
        let file = std::env::current_exe()?;
        let mut base = file.parent().unwrap().parent().unwrap().parent().unwrap();
        if base.to_str().unwrap().ends_with("target") {
            base = base.parent().unwrap();
        }
        Ok(base.to_path_buf())
    }

    fn get_cached_input(&self, solution_day: &SolutionDay) -> Result<String, ClientError> {
        let path = format!("y{}/d{}.txt", solution_day.year, solution_day.day);
        let path = self.cache_dir.join(path);
        Ok(std::fs::read_to_string(path)?)
    }

    fn cache_input(&self, solution_day: &SolutionDay, input: &str) -> Result<(), ClientError> {
        let SolutionDay { year, day, .. } = solution_day;
        let path = self.cache_dir.join(format!("y{}/d{}.txt", year, day));
        fs::create_dir_all(path.parent().unwrap())?;
        let mut file = File::create(path)?;
        file.write_all(input.as_bytes())?;
        Ok(())
    }

    fn download_input(&self, solution_day: &SolutionDay) -> Result<String, ClientError> {
        let SolutionDay { year, day, .. } = solution_day;
        let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
        let cookie = format!("session={}", self.session_token);
        let input = self.client.get(&url).header(COOKIE, cookie).send()?;
        let input = input.error_for_status()?.text()?;
        Ok(input)
    }

    fn post_request(&self, url: &str, params: &Params) -> Result<String, ClientError> {
        let cookie = format!("session={}", self.session_token);
        let req = self.client.post(url).header(COOKIE, cookie).form(&params);
        let response = req.send()?.error_for_status()?.text()?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_input() {
        let client = Client::new().unwrap();
        let solution = SolutionDay {
            year: 2023,
            day: 6,
            part: 1,
        };
        let result = client.get_input(&solution).unwrap();
        assert_eq!(
            result,
            "Time:        44     80     65     72\nDistance:   208   1581   1050   1102\n"
        );
    }

    #[test]
    fn test_input_is_cached() {
        let client = Client::new().unwrap();
        let solution = SolutionDay {
            year: 2023,
            day: 6,
            part: 1,
        };
        client.get_input(&solution).unwrap();
        let result = client.get_cached_input(&solution).unwrap();
        assert_eq!(
            result,
            "Time:        44     80     65     72\nDistance:   208   1581   1050   1102\n"
        );
    }

    #[test]
    fn test_caching() {
        let solution = SolutionDay {
            year: 1000,
            day: 100,
            part: 1,
        };
        let client = Client::new().unwrap();
        client.cache_input(&solution, "test").unwrap();
        let res = client.get_cached_input(&solution).unwrap();
        assert_eq!(res, "test");
    }
}
