use anyhow::anyhow;

use crate::{common::Coord, Input, PResult};

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day18.txt")?;

    println!("18:1 - {}", run_1(&input, 71, 1024)?);
    println!("18:2 - {}", run_2(&input, 71)?);

    Ok(())
}

fn parse(i: Input) -> PResult<Vec<Coord>> {
    let c = nom::combinator::map(
        nom::sequence::separated_pair(
            nom::character::complete::i64,
            nom::bytes::complete::tag(","),
            nom::character::complete::i64,
        ),
        |(x, y)| Coord(y as _, x as _),
    );
    nom::multi::separated_list1(nom::character::complete::newline, c)(i)
}

fn run_1(input: &str, len: usize, num_bytes: usize) -> anyhow::Result<usize> {
    let (_i, mut byte_pos) = parse(input).map_err(|e| anyhow!("{e}"))?;

    byte_pos.truncate(num_bytes);

    let start: Coord = (0, 0).into();
    let end: Coord = (len - 1, len - 1).into();

    let (_path, cost) = pathfinding::directed::astar::astar(
        &start,
        |c| {
            c.neighbors()
                .filter(|c| {
                    c.0 >= 0
                        && c.1 >= 0
                        && c.0 < (len as isize)
                        && c.1 < (len as isize)
                        && !byte_pos.contains(c)
                })
                .map(|c| (c, 1))
        },
        |c| c.manhattan(&end),
        |c| *c == end,
    )
    .ok_or(anyhow!("No path"))?;

    Ok(cost)
}

fn run_2(input: &str, len: usize) -> anyhow::Result<Coord> {
    let (_i, byte_pos) = parse(input).map_err(|e| anyhow!("{e}"))?;

    let start: Coord = (0, 0).into();
    let end: Coord = (len - 1, len - 1).into();

    for l in 1..byte_pos.len() {
        let byte_pos = &byte_pos[..l];

        if pathfinding::directed::astar::astar(
            &start,
            |c| {
                c.neighbors()
                    .filter(|c| {
                        c.0 >= 0
                            && c.1 >= 0
                            && c.0 < (len as isize)
                            && c.1 < (len as isize)
                            && !byte_pos.contains(c)
                    })
                    .map(|c| (c, 1))
            },
            |c| c.manhattan(&end),
            |c| *c == end,
        )
        .is_none()
        {
            return Ok(byte_pos[l - 1]);
        }
    }

    Err(anyhow!("No sol"))
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn day18_run_1() {
        assert_eq!(super::run_1(INPUT, 7, 12).unwrap(), 22);
    }

    #[test]
    fn day18_run_2() {
        assert_eq!(super::run_2(INPUT, 7).unwrap(), (1, 6).into());
    }
}
