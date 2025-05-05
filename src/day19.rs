use anyhow::anyhow;
use nom::Parser;
use rayon::prelude::*;

use crate::{Input, PResult};

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day19.txt")?;

    println!("19:1 - {}", run_1(&input)?);
    println!("19:2 - {}", run_2(&input)?);

    Ok(())
}

type Patterns = Vec<Vec<char>>;

fn parse_patterns(i: Input) -> PResult<Vec<&str>> {
    nom::multi::separated_list1(
        nom::bytes::complete::tag(", "),
        nom::character::complete::alpha1,
    )
    .parse(i)
}

fn parse(i: Input) -> PResult<(Patterns, Vec<String>)> {
    let (i, patterns) = parse_patterns(i)?;
    let (i, _) = nom::character::complete::newline(i)?;
    let (i, _) = nom::character::complete::newline(i)?;

    let designs = i.lines().map(|l| l.to_owned()).collect();
    let patterns = patterns.iter().map(|p| p.chars().collect()).collect();
    Ok(("", (patterns, designs)))
}

fn match_patterns(design: &[char], patterns: &[Vec<char>]) -> bool {
    if design.is_empty() {
        return true;
    }
    for pattern in patterns {
        if design.len() >= pattern.len()
            && &design[..pattern.len()] == pattern
            && match_patterns(&design[pattern.len()..], patterns)
        {
            return true;
        }
    }
    false
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let (_, (patterns, designs)) = parse(input).map_err(|e| anyhow!("{e}"))?;

    let designs: Vec<Vec<char>> = designs.into_iter().map(|l| l.chars().collect()).collect();
    let mut cnt = 0;
    for d in designs {
        if match_patterns(&d, &patterns) {
            cnt += 1;
        }
    }

    Ok(cnt)
}

fn count_match_patterns<'a>(
    design: &'a [char],
    patterns: &[Vec<char>],
    lookup: std::sync::Arc<std::sync::Mutex<std::collections::HashMap<&'a [char], usize>>>,
) -> usize {
    if design.is_empty() {
        return 1;
    }
    if let Some(cnt) = lookup.lock().unwrap().get(design) {
        return *cnt;
    }

    let cnts = patterns.par_iter().map(|pattern| {
        if design.len() >= pattern.len() && &design[..pattern.len()] == pattern {
            count_match_patterns(&design[pattern.len()..], patterns, lookup.clone())
        } else {
            0
        }
    });

    let cnt = cnts.sum();

    lookup.lock().unwrap().insert(design, cnt);

    cnt
}
fn run_2(input: &str) -> anyhow::Result<usize> {
    let (_, (patterns, designs)) = parse(input).map_err(|e| anyhow!("{e}"))?;
    let designs: Vec<Vec<char>> = designs.into_iter().map(|l| l.chars().collect()).collect();
    let lookup: std::collections::HashMap<&[char], usize> = std::collections::HashMap::new();
    let lookup = std::sync::Arc::new(std::sync::Mutex::new(lookup));

    let cnts = designs
        .par_iter()
        .map(|d| count_match_patterns(d, &patterns, lookup.clone()));
    Ok(cnts.sum())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn day19_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 6);
    }

    #[test]
    fn day19_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap(), 16);
    }
}
