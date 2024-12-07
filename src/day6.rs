use std::{collections::HashSet};
use rayon::prelude::*;

use crate::common::{Coord, Dir, Pos};

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day6.txt")?;

    println!("6:1 - {}", run_1(&input)?);
    println!("6:2 - {}", run_2(&input)?);

    Ok(())
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let mut obstacles: HashSet<Coord> = HashSet::new();
    let mut pos = crate::common::Pos {
        coord: (0, 0).into(),
        dir: crate::common::Dir::N,
    };
    let mut num_rows = 0;
    let mut num_cols = 0;
    for (row, line) in input.lines().enumerate() {
        num_rows += 1;
        for (col, ch) in line.chars().enumerate() {
            num_cols = num_cols.max(col + 1);
            match ch {
                '^' => {
                    pos.coord = (row, col).into();
                }
                '#' => {
                    obstacles.insert((row, col).into());
                }
                _ => {}
            }
        }
    }
    let mut visited: HashSet<Coord> = HashSet::new();
    visited.insert(pos.coord);
    loop {
        let next_pos = pos.coord + pos.dir.movement();
        if next_pos.icol() < 0 || next_pos.icol() >= (num_cols as isize) {
            break;
        }
        if next_pos.irow() < 0 || next_pos.irow() >= (num_rows as isize) {
            break;
        }
        if obstacles.contains(&next_pos) {
            pos.dir = pos.dir.turn_right();
            continue;
        }
        visited.insert(next_pos);
        pos.coord = next_pos;
    }

    Ok(visited.len())
}

fn simulate_objects(
    obstacles: &HashSet<Coord>,
    extra_obstacle: Coord,
    start_coord: Coord,
    num_rows: usize,
    num_cols: usize,
) -> bool {
    let mut visited = HashSet::new();
    let mut pos = Pos {
        coord: start_coord,
        dir: Dir::N,
    };
    visited.insert(pos);

    loop {
        if pos.coord.icol() < 0 || pos.coord.icol() > (num_cols as isize) {
            return false;
        }
        if pos.coord.irow() < 0 || pos.coord.irow() > (num_rows as isize) {
            return false;
        }

        let next_pos = pos.coord + pos.dir.movement();
        visited.insert(pos);

        if obstacles.contains(&next_pos) || next_pos == extra_obstacle {
            pos.dir = pos.dir.turn_right();
        } else {
            pos.coord = next_pos;
        }

        if visited.contains(&pos) {
            return true;
        }
    }
}

//1562 - too low
fn run_2(input: &str) -> anyhow::Result<usize> {
    let mut obstacles: HashSet<Coord> = HashSet::new();
    let mut pos = crate::common::Pos {
        coord: (0, 0).into(),
        dir: crate::common::Dir::N,
    };
    let mut num_rows = 0;
    let mut num_cols = 0;
    for (row, line) in input.lines().enumerate() {
        num_rows += 1;
        for (col, ch) in line.chars().enumerate() {
            num_cols = num_cols.max(col + 1);
            match ch {
                '^' => {
                    pos.coord = (row, col).into();
                }
                '#' => {
                    obstacles.insert((row, col).into());
                }
                _ => {}
            }
        }
    }
    let mut visited = Vec::new();

    let start_coord = pos.coord;

    loop {
        visited.push(pos);
        let next_pos = pos.coord + pos.dir.movement();
        if next_pos.icol() < 0 || next_pos.icol() > (num_cols as isize) {
            break;
        }
        if next_pos.irow() < 0 || next_pos.irow() > (num_rows as isize) {
            break;
        }
        if obstacles.contains(&next_pos) {
            pos.dir = pos.dir.turn_right();
            continue;
        }
        pos.coord = next_pos;
    }

    let mut added_obstacles = HashSet::new();
    for v in visited.iter() {
        let v_next = v.move_forward();
        if !obstacles.contains(&v_next.coord) && v_next.coord != start_coord {
            added_obstacles.insert(v_next.coord);
        }
    }

    let cnt = added_obstacles
        .par_iter()
        .filter(|extra_obstacle| {
            simulate_objects(
                &obstacles,
                **extra_obstacle,
                start_coord,
                num_rows,
                num_cols,
            )
        })
        .count();
    Ok(cnt)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn day6_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 41);
    }

    #[test]
    fn day6_run_2() {
        assert_eq!(super::run_2(INPUT).unwrap(), 6);
    }
}
