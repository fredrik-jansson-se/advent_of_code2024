use anyhow::anyhow;
use rayon::prelude::*;

use crate::{common::Coord, Input, PResult};

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day14.txt")?;

    println!("14:1 - {}", run_1(&input, 101, 103)?);
    println!("14:2 - {}", run_2(&input, 101, 103)?);

    Ok(())
}

#[derive(Debug)]
struct Robot {
    pos: Coord,
    vel: Coord,
    positions: Vec<Coord>,
}

fn parse_robot(i: Input) -> PResult<Robot> {
    let (i, _) = nom::bytes::complete::tag("p=")(i)?;
    let (i, x) = nom::character::complete::i64(i)?;
    let (i, _) = nom::bytes::complete::tag(",")(i)?;
    let (i, y) = nom::character::complete::i64(i)?;

    let (i, _) = nom::bytes::complete::tag(" v=")(i)?;
    let (i, vx) = nom::character::complete::i64(i)?;
    let (i, _) = nom::bytes::complete::tag(",")(i)?;
    let (i, vy) = nom::character::complete::i64(i)?;
    Ok((
        i,
        Robot {
            pos: (y as i32, x as i32).into(),
            vel: (vy as i32, vx as i32).into(),
            positions: Vec::new(),
        },
    ))
}

fn parse(i: Input) -> PResult<Vec<Robot>> {
    nom::multi::separated_list1(nom::character::complete::newline, parse_robot)(i)
}

fn run_1(input: &str, width: isize, height: isize) -> anyhow::Result<usize> {
    let (_, mut robots) = parse(input).map_err(|e| anyhow!("{e}"))?;

    robots.par_iter_mut().for_each(|robot| {
        let mut cur_pos = robot.pos;
        loop {
            robot.positions.push(cur_pos);
            cur_pos = cur_pos + robot.vel;
            cur_pos.0 = (cur_pos.0 + height) % height;
            cur_pos.1 = (cur_pos.1 + width) % width;
            assert!(cur_pos.0 >= 0);
            assert!(cur_pos.1 >= 0);

            if robot.positions.contains(&cur_pos) {
                break;
            }
        }
    });

    let robot_positions: Vec<_> = robots
        .iter()
        .map(|r| r.positions[100 % r.positions.len()])
        .collect();

    let q_height = height / 2;
    let q_width = width / 2;
    let q_upper = height - q_height;
    let q_right = width - q_width;

    let q1 = robot_positions
        .iter()
        .filter(|p| p.irow() < q_height && p.icol() < q_width)
        .count();
    let q2 = robot_positions
        .iter()
        .filter(|p| p.irow() < q_height && p.icol() >= q_right)
        .count();

    let q3 = robot_positions
        .iter()
        .filter(|p| p.irow() >= q_upper && p.icol() < q_width)
        .count();
    let q4 = robot_positions
        .iter()
        .filter(|p| p.irow() >= q_upper && p.icol() >= q_right)
        .count();
    dbg! {q1};
    dbg! {q2};
    dbg! {q3};
    dbg! {q4};

    Ok(q1 * q2 * q3 * q4)
}

fn run_2(input: &str, width: isize, height: isize) -> anyhow::Result<usize> {
    let (_, mut robots) = parse(input).map_err(|e| anyhow!("{e}"))?;

    robots.par_iter_mut().for_each(|robot| {
        let mut cur_pos = robot.pos;
        loop {
            robot.positions.push(cur_pos);
            cur_pos = cur_pos + robot.vel;
            cur_pos.0 = (cur_pos.0 + height) % height;
            cur_pos.1 = (cur_pos.1 + width) % width;
            assert!(cur_pos.0 >= 0);
            assert!(cur_pos.1 >= 0);

            if robot.positions.contains(&cur_pos) {
                break;
            }
        }
    });
    let mut i = 0;
    loop {
        i+=1;
        if i == 1_000_000 {
            break;
        }
        let robot_positions: Vec<_> = robots
            .iter()
            .map(|r| r.positions[i % r.positions.len()])
            .collect();
        let q_height = height / 2;
        let q_width = width / 2;
        let q_upper = height - q_height;
        let q_right = width - q_width;

        let q1 = robot_positions
            .iter()
            .filter(|p| p.irow() < q_height && p.icol() < q_width)
            .count();
        let q2 = robot_positions
            .iter()
            .filter(|p| p.irow() < q_height && p.icol() >= q_right)
            .count();

        let q3 = robot_positions
            .iter()
            .filter(|p| p.irow() >= q_upper && p.icol() < q_width)
            .count();
        let q4 = robot_positions
            .iter()
            .filter(|p| p.irow() >= q_upper && p.icol() >= q_right)
            .count();
        if q1 == q2 && q3 == q4 {
            println!("I: {i}");
            for row in 0..height {
                for col in 0..width {
                    let c: Coord = (row, col).into();
                    if robot_positions.contains(&c) {
                        print!("X");
                    } else {
                        print!(" ");
                    }
                }
                println!();
            }
            //let mut x = String::new();
            println!("{i}");
            //let _ = std::io::stdin().lock().read_line(&mut x);
        }
    }
    Ok(0)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn day14_run_1() {
        assert_eq!(super::run_1(INPUT, 11, 7).unwrap(), 12);
    }

    #[test]
    fn day14_run_2() {}
}
