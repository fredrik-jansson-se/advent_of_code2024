use anyhow::anyhow;
use nom::{character::complete::newline, sequence::separated_pair};

use crate::PResult;

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day1.txt")?;

    println!("1:1 - {}", run_1(&input)?);
    println!("1:2 - {}", run_2(&input)?);

    Ok(())
}

fn run_1(input: &str) -> anyhow::Result<i64> {
    let (_i, res) = parse(input).map_err(|e| anyhow!("{e}"))?;

    let mut left: Vec<_> = res.iter().map(|(a, _b)| a).collect();
    let mut right: Vec<_> = res.iter().map(|(_a, b)| b).collect();
    left.sort();
    right.sort();

    Ok(left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| (**a - **b).abs())
        .sum())
}

fn run_2(_input: &str) -> anyhow::Result<usize> {
    todo!()
}

fn parse(i: crate::Input) -> PResult<Vec<(i64, i64)>> {
    let row = separated_pair(
        nom::character::complete::i64,
        nom::character::complete::space1,
        nom::character::complete::i64,
    );

    nom::multi::separated_list1(newline, row)(i)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn day1_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 11);
    }

    #[test]
    fn day1_run_2() {}
}
