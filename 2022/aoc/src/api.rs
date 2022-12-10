use dirs;
use regex::Regex;
use reqwest::header;
use serde;
use std::error::Error;
use std::fmt;
use std::fs;

#[derive(Debug)]
enum SendAnswerError {
    Wrong(u64),
    Waiting(u8, u8),
    AlreadyAnswered,
}

impl fmt::Display for SendAnswerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            SendAnswerError::Wrong(answer) => {
                write!(f, "The submitted answer {} is WRONG. Try again.", answer)
            }
            SendAnswerError::Waiting(minutes, seconds) => {
                write!(f, "You've answered too recently! Go grab a coffee, wait {} minutes and {} seconds, and try again ☕", minutes, seconds)
            }
            SendAnswerError::AlreadyAnswered => {
                write!(f, "Looks like you solved this one already, bud.")
            }
        }
    }
}

impl Error for SendAnswerError {}

pub struct Client {
    http_client: reqwest::blocking::Client,
    base_url: String,
}

pub enum PuzzleId {
    PartOne = 1,
    PartTwo = 2,
}

#[derive(serde::Serialize)]
struct Solution {
    level: u8,
    answer: String,
}

impl Client {
    pub fn new(year: u16, day: u8) -> Self {
        let cookie = format!("session={}", read_cookie());

        let mut headers = header::HeaderMap::new();
        headers.insert(header::COOKIE, cookie.parse().unwrap());

        let client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        Self {
            http_client: client,
            base_url: format!("https://adventofcode.com/{year}/day/{day}/"),
        }
    }

    pub fn get_input(&self) -> Result<String, reqwest::Error> {
        let url = self.base_url.clone() + "input";

        let response = self.http_client.get(url).send()?;
        Ok(response.text()?)
    }

    pub fn send_answer(&self, part: PuzzleId, answer: u64) -> Result<(), Box<dyn Error>> {
        let url = self.base_url.clone() + "answer";

        let response = self
            .http_client
            .post(url)
            .form(&[("level", part as u64), ("answer", answer)])
            .send()?
            .text()?;
        println!("{}", response);
        return Self::check_answer(answer, response);
    }

    fn check_answer(answer: u64, response: String) -> Result<(), Box<dyn Error>> {
        let timeout_expr = Regex::new(r"You have (\d*)m* *(\d+)s left to wait").unwrap();
        match response.as_str() {
            response if response.contains("not the right answer") => {
                return Err(Box::new(SendAnswerError::Wrong(answer)));
            }
            response if response.contains("Did you already complete it") => {
                return Err(Box::new(SendAnswerError::AlreadyAnswered));
            }
            response if timeout_expr.is_match(response) => {
                let captures = timeout_expr.captures(response).unwrap();
                return Err(Box::new(SendAnswerError::Waiting(
                    captures.get(1).unwrap().as_str().parse().unwrap(),
                    captures.get(2).unwrap().as_str().parse().unwrap(),
                )));
            }
            _ => Ok(()),
        }
    }
}

fn read_cookie() -> String {
    let cookie_location = format!(
        "{}{}",
        dirs::home_dir().unwrap().to_str().unwrap(),
        "/.config/aoc_cookie"
    );

    String::from(
        fs::read_to_string(cookie_location.clone())
            .expect(format!("Unable to read file {cookie_location}").as_str())
            .trim(),
    )
}
