use crate::utils::*;
use std::hash::Hash;

pub fn parse_day1(input: &str) -> Result<(Vec<i32>, Vec<i32>)> {
    let table: Vec<Vec<i32>> = tabulate(input).iter()
        .map(|line| line.iter()
            .map(|cell| cell.parse::<i32>().expect("could not parse int"))
            .collect()
        )
        .collect();

    let list1 = extract_column(&table, 0);
    let list2 = extract_column(&table, 1);

    Ok((list1, list2))
}

#[allow(dead_code)]
pub async fn part1() -> Result<i32> {

    let data = fetch_input(1).await?;

    let start = std::time::Instant::now();
    
    let (mut list1, mut list2) = parse_day1(&data)?;

    list1.sort();
    list2.sort();

    let result: i32 = list1.iter()
        .zip(list2.iter())
        .map(|(a, b)| b - a)
        .map(|diff| diff.abs())
        .sum();

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);

    Ok(result)
}

#[allow(dead_code)]
pub async fn part2() -> Result<i32> {
    let data = fetch_input(1).await?;
    // let data = fetch_test_inputs(1)
    //     .await?
    //     [0]
    //     .clone();

    let start = std::time::Instant::now();
    
    let (list1, list2) = parse_day1(&data)?;

    let list2_freq = frequency_map(&list2);
    
    let result: i32 = list1.iter()
        .map(|val| {
            let frequency = *list2_freq.get(val).unwrap_or(&0);
            let similarity_score = val * frequency as i32;

            return similarity_score;
        })
        .sum();

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    
    Ok(result)
}

fn increment_key<T: Copy + Eq + Hash>(mut map: std::collections::HashMap<T, usize>, key: &T) -> std::collections::HashMap<T, usize> {
    let count = map.entry(*key).or_insert(0);
    *count += 1;
    map
}

fn frequency_map<T: Copy + Eq + Hash>(list: &Vec<T>) -> std::collections::HashMap<T, usize> {
    list.iter()
        .fold(std::collections::HashMap::new(), increment_key)
}