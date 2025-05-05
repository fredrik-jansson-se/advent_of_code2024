use anyhow::anyhow;
use nom::Parser;

use crate::{Input, PResult};

type Updates = Vec<usize>;

struct PageInput {
    orders: Vec<(usize, usize)>,
    updates: Vec<Updates>,
}

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day5.txt")?;

    println!("5:1 - {}", run_1(&input)?);
    println!("5:2 - {}", run_2(&input)?);

    Ok(())
}

fn parse(i: Input) -> PResult<PageInput> {
    let (i, orders) =
        nom::multi::separated_list1(nom::character::complete::newline, parse_pair).parse(i)?;

    let (i, _) = nom::multi::many1(nom::character::complete::newline).parse(i)?;
    let (i, updates) =
        nom::multi::separated_list1(nom::character::complete::newline, parse_updates).parse(i)?;
    Ok((i, PageInput { orders, updates }))
}

fn parse_pair(i: Input) -> PResult<(usize, usize)> {
    let (i, v1) = nom::combinator::map(nom::character::complete::u64, |v| v as usize).parse(i)?;
    let (i, _) = nom::bytes::complete::tag("|")(i)?;
    let (i, v2) = nom::combinator::map(nom::character::complete::u64, |v| v as usize).parse(i)?;

    Ok((i, (v1, v2)))
}

fn parse_updates(i: Input) -> PResult<Updates> {
    nom::multi::separated_list1(
        nom::bytes::complete::tag(","),
        nom::combinator::map(nom::character::complete::u64, |v| v as usize),
    ).parse(i)
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let (_i, pi) = parse(input).map_err(|e| anyhow!("{e}"))?;
    let ok = pi
        .updates
        .iter()
        .filter(|o| o.is_sorted_by(|a, b| compare(&pi.orders, *a, *b) == std::cmp::Ordering::Less));

    let sum = ok
        .map(|u| u[u.len() / 2])
        .sum();
    Ok(sum)
}

fn compare(orders: &[(usize, usize)], a: usize, b: usize) -> std::cmp::Ordering {
    if orders.contains(&(a, b)) {
        std::cmp::Ordering::Less
    } else {
        std::cmp::Ordering::Greater
    }
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let (_i, mut pi) = parse(input).map_err(|e| anyhow!("{e}"))?;
    let mut sum = 0;

    for o in pi.updates.iter_mut() {
        if !o.is_sorted_by(|a, b| compare(&pi.orders, *a, *b) == std::cmp::Ordering::Less) {
            o.sort_by(|a,b| compare(&pi.orders, *a, *b));

            sum += o[o.len()/2];
        }
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn day5_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 143);
    }

    #[test]
    fn day5_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap(), 123);
    }
}
