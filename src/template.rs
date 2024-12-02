use crate::utils::*;

fn parse(input: &str) -> Result<(Vec<i32>, Vec<i32>)> {
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

pub async fn part1(data: &str) -> Result<i32> {
    Ok(0)
}

pub async fn part2(data: &str) -> Result<i32> {    
    Ok(0)
}