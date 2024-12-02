use anyhow::anyhow;
use nom::{character::complete::newline, multi::separated_list1};

use crate::{Input, PResult};

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day2.txt")?;

    println!("2:1 - {}", run_1(&input)?);
    println!("2:2 - {}", run_2(&input)?);

    Ok(())
}

type Report = Vec<i64>;

fn parse(i: Input) -> PResult<Vec<Report>> {
    let report = separated_list1(
        nom::character::complete::space1,
        nom::character::complete::i64,
    );
    separated_list1(newline, report)(i)
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let (i, reports) = parse(input).map_err(|e| anyhow!("{e}"))?;
    assert!(i.len() <= 1);

    let mut safe = 0;

    'reports: for report in reports {
        let decreasing = report[1] < report[0];
        for l in report.windows(2) {
            let a = l[0];
            let b = l[1];
            if decreasing && a < b {
                continue 'reports;
            }
            if !decreasing && a > b {
                continue 'reports;
            }
            if !(1..=3).contains(&(a - b).abs()) {
                continue 'reports;
            }
        }
        safe += 1;
    }

    Ok(safe)
}

// 553 - too low
// 560 - too low
fn run_2(input: &str) -> anyhow::Result<usize> {
    let (i, reports) = parse(input).map_err(|e| anyhow!("{e}"))?;
    assert!(i.len() <= 1);

    let mut safe = 0;

    'reports: for report in reports {
        let diffs: Vec<i64> = report.windows(2).map(|v| v[0] - v[1]).collect();

        if diffs.iter().all(|v| (-3..0).contains(v)) ||
           diffs.iter().all(|v| (1..4).contains(v)) {
            safe += 1;
            continue;
        }
        for i in 0..report.len() {
            let mut report = report.clone();
            report.remove(i);
            let diffs: Vec<i64> = report.windows(2).map(|v| v[0] - v[1]).collect();
            if diffs.iter().all(|v| (-3..0).contains(v)) ||
                diffs.iter().all(|v| (1..4).contains(v)) {
                    safe += 1;
                    continue 'reports;
            }
        }
    }

    Ok(safe)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn day2_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 2);
    }

    #[test]
    fn day2_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap(), 4);
    }
}
