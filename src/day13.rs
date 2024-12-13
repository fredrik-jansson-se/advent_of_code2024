use anyhow::anyhow;
use rayon::prelude::*;

use crate::{common::Coord, Input, PResult};

#[derive(Debug)]
struct Game {
    a: Coord,
    b: Coord,
    prize: Coord,
}

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day13.txt")?;

    println!("13:1 - {}", run_1(&input)?);
    println!("13:2 - {}", run_2(&input)?);

    Ok(())
}

fn parse_game(i: Input) -> PResult<Game> {
    let (i, _) = nom::bytes::complete::tag("Button A: X+")(i)?;
    let (i, ax) = nom::character::complete::u64(i)?;
    let (i, _) = nom::bytes::complete::tag(", Y+")(i)?;
    let (i, ay) = nom::character::complete::u64(i)?;
    let (i, _) = nom::character::complete::newline(i)?;

    let (i, _) = nom::bytes::complete::tag("Button B: X+")(i)?;
    let (i, bx) = nom::character::complete::u64(i)?;
    let (i, _) = nom::bytes::complete::tag(", Y+")(i)?;
    let (i, by) = nom::character::complete::u64(i)?;
    let (i, _) = nom::character::complete::newline(i)?;

    let (i, _) = nom::bytes::complete::tag("Prize: X=")(i)?;
    let (i, px) = nom::character::complete::u64(i)?;
    let (i, _) = nom::bytes::complete::tag(", Y=")(i)?;
    let (i, py) = nom::character::complete::u64(i)?;
    let (i, _) = nom::character::complete::newline(i)?;
    Ok((
        i,
        Game {
            a: (ay, ax).into(),
            b: (by, bx).into(),
            prize: (py, px).into(),
        },
    ))
}

fn parse(i: Input) -> PResult<Vec<Game>> {
    nom::multi::separated_list1(nom::character::complete::newline, parse_game)(i)
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let (_, games) = parse(input).map_err(|e| anyhow!("{e}"))?;

    let res = games.par_iter().filter_map(|game| {
        let start: ((usize, usize), Coord) = ((0, 0), (0, 0).into());
        let res = pathfinding::directed::astar::astar(
            &start,
            |((a_pushes, b_pushes), c)| {
                let mut res = Vec::new();
                if *a_pushes < 100 {
                    res.push((((a_pushes + 1, *b_pushes), *c + game.a), 3));
                }
                if *b_pushes < 100 {
                    res.push((((*a_pushes, *b_pushes + 1), *c + game.b), 1));
                }
                res
            },
            |(_, c)| c.manhattan(&game.prize),
            |(_, c)| *c == game.prize,
        );
        res.map(|(_, t)| t)
    });

    Ok(res.sum())
}

// too low 81705094267126
fn run_2(input: &str) -> anyhow::Result<usize> {
    let (_, games) = parse(input).map_err(|e| anyhow!("{e}"))?;

    let res = games.par_iter().filter_map(|game| {
        let start: Coord = (0, 0).into();
        let finish = game.prize + (10000000000000usize, 10000000000000usize).into();

        let a = nalgebra::matrix![game.a.col() as f64, game.b.col() as f64; game.a.row() as f64, game.b.row() as f64];
        let c = nalgebra::matrix![finish.col() as f64; finish.row() as f64];
        if let Some(sol) = a.lu().solve(&c) {
            let a = sol[0].round() as usize;
            let b = sol[1].round() as usize;
            if (start + a*game.a + b*game.b) == finish {
                return Some(a * 3 + b);
            }
        }
        None
    });

    Ok(res.sum())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn day13_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 480);
    }
}
