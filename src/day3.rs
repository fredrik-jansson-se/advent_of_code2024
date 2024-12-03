use nom::bytes::complete::tag;

use crate::{Input, PResult};

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day3.txt")?;

    println!("3:1 - {}", run_1(&input)?);
    println!("3:2 - {}", run_2(&input)?);

    Ok(())
}

#[derive(Debug)]
enum Op {
    Mul(i64, i64),
    Do,
    Dont,
}

fn parse_mul_1(i: Input) -> PResult<Op> {
    let (i, _) = tag("mul(")(i)?;
    let (i, v1) = nom::character::complete::i64(i)?;
    let (i, _) = tag(",")(i)?;
    let (i, v2) = nom::character::complete::i64(i)?;
    let (i, _) = tag(")")(i)?;
    Ok((i, Op::Mul(v1, v2)))
}

fn parse_do(i: Input) -> PResult<Op> {
    let (i, _) = tag("do()")(i)?;
    Ok((i, Op::Do))
}

fn parse_dont(i: Input) -> PResult<Op> {
    let (i, _) = tag("don't()")(i)?;
    Ok((i, Op::Dont))
}

fn parse_1(i: &str, acc: &mut Vec<(i64, i64)>) {
    if i.is_empty() {
        return;
    }
    if let Ok((new_i, Op::Mul(a, b))) = parse_mul_1(i) {
        acc.push((a, b));
        parse_1(new_i, acc)
    } else {
        parse_1(&i[1..], acc)
    }
}

fn parse_2(i: &str, acc: &mut Vec<Op>) {
    if i.is_empty() {
        return;
    }
    if let Ok((new_i, op)) = nom::branch::alt((parse_mul_1, parse_dont, parse_do))(i) {
        acc.push(op);
        parse_2(new_i, acc)
    } else {
        parse_2(&i[1..], acc)
    }
}

fn run_1(input: &str) -> anyhow::Result<i64> {
    let mut acc = Vec::new();
    parse_1(input, &mut acc);
    Ok(acc.iter().map(|(a, b)| a * b).sum())
}

fn run_2(input: &str) -> anyhow::Result<i64> {
    let mut acc = Vec::new();
    parse_2(input, &mut acc);
    let mut s = 0;
    let mut enabled = true;
    for op in acc {
        match op {
            Op::Do => enabled = true,
            Op::Dont => enabled = false,
            Op::Mul(a, b) if enabled => {
                s += a * b;
            }
            _ => (),
        }
    }

    Ok(s)
}

#[cfg(test)]
mod tests {
    const INPUT_1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    #[test]
    fn day3_run_1() {
        assert_eq!(super::run_1(INPUT_1).unwrap(), 161);
    }
    const INPUT_2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn day3_run_2() {
        assert_eq!(super::run_2(INPUT_2).unwrap(), 48);
    }
}
