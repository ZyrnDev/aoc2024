use crate::utils::*;


#[derive(Debug)]
struct Puzzle {
    reports: Vec<Report>
}


#[derive(Debug)]
struct Report {
    levels: Vec<Level>
}

type Level = i32;

const MINIMUM_DELTA: i32 = 1;
const MAXIMUM_DELTA: i32 = 3;

const fn is_valid_delta(delta: i32) -> bool {
    delta >= MINIMUM_DELTA && delta <= MAXIMUM_DELTA
}

type ReportValidator = fn(&Report) -> bool;

impl Puzzle {
    fn new(reports: Vec<Report>) -> Puzzle {
        Puzzle { reports: reports }
    }
    
    fn valid_reports(&self, validator: ReportValidator) -> usize {
        self.reports.iter()
            .filter(|report| validator(*report))
            .count() 
    }
}

impl Report {
    fn new(levels: Vec<Level>) -> Report {
        Report { levels: levels }
    }

    fn is_valid(&self) -> bool {
        self.levels.windows(2)
                .map(|window| window[1] - window[0])
                .all(is_valid_delta)
            ||
        self.levels.windows(2)
                .map(|window| window[0] - window[1])
                .all(is_valid_delta)
    }

    fn is_fixable(&self) -> bool {
        for i in 0..self.levels.len() {
            let levels = self.levels.iter().enumerate()
                .filter(|(j,_)| *j != i)
                .map(|(_,level)| *level)
                .collect();

            let report = Report::new(levels);
            if report.is_valid() {
                return true;
            }
        }

        false
    }
}

fn parse(input: &str) -> Result<Puzzle> {
    let reports: Vec<Report> = tabulate(input).iter()
        .map(parse_vec)
        .map(Report::new)
        .collect();

    Ok(Puzzle::new(reports))
}

#[allow(dead_code)]
pub async fn part1(data: &str) -> Result<i32> {
    let puzzle = parse(data)?;

    Ok(puzzle.valid_reports(|report| report.is_valid()) as i32)
}

#[allow(dead_code)]
pub async fn part2(data: &str) -> Result<i32> {    
    let puzzle = parse(data)?;

    Ok(puzzle.valid_reports(|report| report.is_fixable()) as i32)
}