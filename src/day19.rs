use anyhow::anyhow;

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
    )(i)
}

fn parse(i: Input) -> PResult<(Patterns, Vec<String>)> {
    let (i, patterns) = parse_patterns(i)?;
    let (i, _) = nom::character::complete::newline(i)?;
    let (i, _) = nom::character::complete::newline(i)?;

    let designs = i.lines().map(|l| l.to_owned()).collect();
    let patterns = patterns.iter().map(|p| p.chars().collect()).collect();
    Ok(("", (patterns, designs)))
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let (_, (mut patterns, designs)) = parse(input).map_err(|e| anyhow!("{e}"))?;
    patterns.sort_by_key(|a| std::cmp::Reverse(a.len()));
    let designs: Vec<Vec<char>> = designs.into_iter().map(|l| l.chars().collect()).collect();
    let cnt = 0;
    for mut _d in designs {
        for p in &patterns {
          let _p_len = p.len();

        }
    }

    Ok(cnt)
}

fn run_2(_input: &str) -> anyhow::Result<usize> {
    todo!()
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
    #[ignore]
    fn day19_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 6);
    }

    #[test]
    fn day19_run_2() {}
}
