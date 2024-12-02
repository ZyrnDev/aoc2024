mod utils;
mod day1;
mod day2;

use utils::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let day = 2;

    // let data = utils::fetch_test_inputs(day).await?;
    // let data = data[0].clone();

    let data = utils::fetch_input(day).await?;

    let start = std::time::Instant::now();
    let result = day2::part1(&data).await?;
    let duration = start.elapsed();
    println!("[{}µs] Day {} Part 1: {}", duration.as_micros(), day, result);

    let start = std::time::Instant::now();
    let result = day2::part2(&data).await?;
    let duration = start.elapsed();
    println!("[{}µs] Day {} Part 2: {}", duration.as_nanos(), day, result);

    Ok(())
}

