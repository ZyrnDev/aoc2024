#![allow(dead_code)]

use std::fmt::Debug;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn to_lines(data: &str) -> Vec<&str> {
    data.lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>()
}

pub fn to_columns(lines: Vec<&str>) -> Vec<Vec<&str>> {
    lines.into_iter()
        .map(|line| 
            line.split_ascii_whitespace()
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect::<Vec<&str>>()
        )
        .collect()
}

pub fn parse_vec(vec: &Vec<&str>) -> Vec<i32> {
    vec.into_iter()
        .map(|s| 
            s
             .parse::<i32>()
             .expect(format!("could not parse i32 from {}", s).as_str()))
        .collect()
}

pub fn tabulate(input: &str) -> Vec<Vec<&str>> {
    to_columns(to_lines(input))
}

pub fn debug<T: Debug>(value: &T) -> &T {
    println!("{:?}", value);
    value
}

pub fn extract_column<T: Copy>(table: &Vec<Vec<T>>, index: usize) -> Vec<T> {
    table.into_iter()
        .map(|line| *line.get(index).expect("Could not get element"))
        .collect()
}

pub async fn fetch_input(day: i32) -> Result<String> {
    let cookie = include_str!("../cookie.dat");

    let client = reqwest::Client::new();
    let data = client.get(format!("https://adventofcode.com/2024/day/{}/input", day))
        .header("cookie", cookie)
        .send()
        .await?
        .text()
        .await?;

    Ok(data)
}

pub async fn fetch_test_inputs(day: i32) -> Result<Vec<String>> {
    let data = reqwest::get(format!("https://adventofcode.com/2024/day/{}", day))
        .await?
        .text()
        .await?;

    let dom = tl::parse(&data, tl::ParserOptions::default())?;
    let parser = dom.parser();

    let pre_blocks = dom.query_selector("pre").ok_or("Could not find pre tag")?
        .map(|handle| handle.get(parser).unwrap().inner_text(parser))
        .map(|text| text.to_string())
        .collect::<Vec<String>>();  

    Ok(pre_blocks)
}