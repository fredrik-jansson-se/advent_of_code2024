use anyhow::anyhow;
use rayon::prelude::*;

use crate::{Input, PResult};

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day7.txt")?;

    println!("7:1 - {}", run_1(&input)?);
    println!("7:2 - {}", run_2(&input)?);

    Ok(())
}

struct Problem {
    ans: u64,
    terms: Vec<u64>,
}

fn can_solve_problem(p: &Problem, mut operators: usize) -> bool {
    let mut res = p.terms[0];
    for term in p.terms.iter().skip(1) {
        if (operators & 1) == 0 {
            res += term;
        } else {
            res *= term;
        }
        operators >>= 1;
    }
    res == p.ans
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let (_, problems) = parse(input).map_err(|e| anyhow!("{e}"))?;

    let mut res = 0;
    for p in &problems {
        let num_operators = 2usize.pow(p.terms.len() as u32 - 1);
        if (0..=num_operators)
            .into_par_iter()
            .any(|operators| can_solve_problem(p, operators))
        {
            res += p.ans;
        }
    }

    Ok(res as _)
}

fn can_solve_problem_2(p: &Problem, mut operators: usize) -> bool {
    let mut res = p.terms[0];
    for term in p.terms.iter().skip(1) {
        let m = operators % 3;
        operators /= 3;
        if m == 0 {
            res += term;
        } else if m == 1 {
            res *= term;
        } else {
            res = format!("{res}{}", term).parse().unwrap();
        }
    }
    res == p.ans
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let (_, problems) = parse(input).map_err(|e| anyhow!("{e}"))?;

    let mut res = 0;
    for p in &problems {
        let num_operators = 3usize.pow(p.terms.len() as u32 - 1);
        if (0..=num_operators)
            .into_par_iter()
            .any(|operators| can_solve_problem_2(p, operators))
        {
            res += p.ans;
        }
    }

    Ok(res as _)
}

fn parse_problem(i: Input) -> PResult<Problem> {
    let (i, ans) = nom::character::complete::u64(i)?;
    let (i, _) = nom::bytes::complete::tag(": ")(i)?;
    let (i, terms) = nom::multi::separated_list1(
        nom::character::complete::space1,
        nom::character::complete::u64,
    )(i)?;
    Ok((i, Problem { ans, terms }))
}

fn parse(i: Input) -> PResult<Vec<Problem>> {
    nom::multi::separated_list1(nom::character::complete::newline, parse_problem)(i)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn day7_can_solve() {
        assert!(super::can_solve_problem(
            &super::Problem {
                ans: 190,
                terms: vec![10, 19]
            },
            1
        ));
        assert!(super::can_solve_problem(
            &super::Problem {
                ans: 3267,
                terms: vec![81, 40, 27]
            },
            1
        ));

        assert!(super::can_solve_problem_2(
            &super::Problem {
                ans: 156,
                terms: vec![15, 6]
            },
            2
        ));
        assert!(super::can_solve_problem_2(
            &super::Problem {
                ans: 7290,
                terms: vec![6, 8, 6, 15]
            },
            961
        ));
        assert!(super::can_solve_problem_2(
            &super::Problem {
                ans: 192,
                terms: vec![17, 8, 14]
            },
            2
        ));
    }

    #[test]
    fn day7_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 3749);
    }

    #[test]
    fn day7_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap(), 11387);
    }
}
