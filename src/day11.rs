use std::collections::HashMap;

use anyhow::anyhow;
use rayon::prelude::*;

use crate::{Input, PResult};

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day11.txt")?;

    println!("11:1 - {}", run_1(&input)?);
    println!("11:2 - {}", run_2(&input)?);

    Ok(())
}

fn num_digits(v: u64) -> u32 {
    if v == 0 {
        1
    } else {
        (v as f64).log10().floor() as u32 + 1
    }
}

fn update(v: u64) -> (u64, Option<u64>) {
    if v == 0 {
        return (1, None);
    }
    let num_d = num_digits(v);
    if num_d % 2 == 0 {
        let div = 10usize.pow(num_d / 2) as u64;
        let upper = v / div;
        let lower = v % div;
        return (upper, Some(lower));
    }
    (2024 * v, None)
}

fn parse(i: Input) -> PResult<Vec<u64>> {
    nom::multi::separated_list1(
        nom::character::complete::space1,
        nom::character::complete::u64,
    )(i)
}

fn expand(v: u64, depth: usize, max_depth: usize) -> usize {
    let (a, b) = update(v);

    if depth == max_depth {
        1 + b.map(|_| 1).unwrap_or(0)
    } else {
        expand(a, depth + 1, max_depth) + b.map(|v| expand(v, depth + 1, max_depth)).unwrap_or(0)
    }
}
fn expand2(
    v: u64,
    depth: usize,
    max_depth: usize,
    lookup: &mut HashMap<(u64, usize), usize>,
) -> usize {
    let (a, b) = update(v);

    if depth == max_depth {
        1 + b.map(|_| 1).unwrap_or(0)
    } else {
        let b = b
            .map(|b| {
                if let Some(x) = lookup.get(&(b, depth)) {
                    *x
                } else {
                    let v = expand2(b, depth + 1, max_depth, lookup);
                    lookup.insert((b, depth), v);
                    v
                }
            })
            .unwrap_or(0);
        let a = if let Some(x) = lookup.get(&(a, depth)) {
            *x
        } else {
            let v = expand2(a, depth + 1, max_depth, lookup);
            lookup.insert((a, depth), v);
            v
        };
        a + b
    }
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let (_, nums) = parse(input).map_err(|e| anyhow!("{e}"))?;
    Ok(nums.par_iter().map(|v| expand(*v, 0, 24)).sum())
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let (_, nums) = parse(input).map_err(|e| anyhow!("{e}"))?;
    let mut lookup = HashMap::new();
    Ok(nums.iter().map(|v| expand2(*v, 0, 74, &mut lookup)).sum())
}

#[cfg(test)]
mod tests {
    #[test]
    fn day11_num_digits() {
        assert_eq!(super::num_digits(1), 1);
        assert_eq!(super::num_digits(76), 2);
        assert_eq!(super::num_digits(176), 3);
        assert_eq!(super::num_digits(9176), 4);
    }
    #[test]
    fn day11_update() {
        assert_eq!(super::update(10), (1, Some(0)));
        assert_eq!(super::update(99), (9, Some(9)));
    }
    #[test]
    fn day11_run_1() {
        assert_eq!(super::run_1("125 17").unwrap(), 55312);
    }
}
