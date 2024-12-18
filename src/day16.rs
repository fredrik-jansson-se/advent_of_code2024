use std::collections::HashSet;

use crate::common::{Coord, Dir, Pos};

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day16.txt")?;

    println!("16:1 - {}", run_1(&input)?);
    println!("16:2 - {}", run_2(&input)?);

    Ok(())
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let mut start: Coord = (0, 0).into();
    let mut end: Coord = (0, 0).into();
    let mut walls: HashSet<Coord> = HashSet::new();
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                walls.insert((row, col).into());
            } else if ch == 'S' {
                start = (row, col).into();
            } else if ch == 'E' {
                end = (row, col).into();
            }
        }
    }

    let start_pos = Pos {
        dir: Dir::E,
        coord: start,
    };

    let (_path, cost) = pathfinding::directed::astar::astar(
        &start_pos,
        |p| {
            let mut res = Vec::with_capacity(3);
            let fwd = p.move_forward();
            if !walls.contains(&fwd.coord) {
                res.push((fwd, 1));
            }
            res.push((p.turn_right(), 1000));
            res.push((p.turn_left(), 1000));

            res
        },
        |p| p.coord.manhattan(&end),
        |p| p.coord == end,
    )
    .ok_or(anyhow::anyhow!("No solution"))?;

    Ok(cost)
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let mut start: Coord = (0, 0).into();
    let mut end: Coord = (0, 0).into();
    let mut walls: HashSet<Coord> = HashSet::new();
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                walls.insert((row, col).into());
            } else if ch == 'S' {
                start = (row, col).into();
            } else if ch == 'E' {
                end = (row, col).into();
            }
        }
    }

    let start_pos = Pos {
        dir: Dir::E,
        coord: start,
    };


    let (paths, _cost) = pathfinding::directed::astar::astar_bag(
        &start_pos,
        |p| {
            let mut res = Vec::with_capacity(3);
            let fwd = p.move_forward();
            if !walls.contains(&fwd.coord) {
                res.push((fwd, 1));
            }
            res.push((p.turn_right(), 1000));
            res.push((p.turn_left(), 1000));

            res
        },
        |p| p.coord.manhattan(&end),
        |p| p.coord == end,
    )
    .ok_or(anyhow::anyhow!("No solution"))?;

    let mut tiles: HashSet<Coord> = HashSet::new();
    for p in paths {
        for pos in p {
            tiles.insert(pos.coord);
        }
    }

    Ok(tiles.len())
}

#[cfg(test)]
mod tests {
    const INPUT_1: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const INPUT_2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn day16_run_1() {
        assert_eq!(super::run_1(INPUT_1).unwrap(), 7036);
        assert_eq!(super::run_1(INPUT_2).unwrap(), 11048);
    }

    #[test]
    fn day16_run_2() {
        assert_eq!(super::run_2(INPUT_1).unwrap(), 45);
        assert_eq!(super::run_2(INPUT_2).unwrap(), 64);
    }
}
