use dirs;
use reqwest::header;
use std::fs;

pub fn get(year: u16, day: u8) -> Result<String, reqwest::Error> {
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let cookie = format!("session={}", read_cookie());

    let mut headers = header::HeaderMap::new();
    headers.insert(header::COOKIE, cookie.parse().unwrap());

    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()?;

    let response = client.get(url).send()?;
    Ok(response.text()?)
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
