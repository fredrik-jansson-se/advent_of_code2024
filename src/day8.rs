use std::collections::{HashMap, HashSet};

use crate::common::Coord;

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day8.txt")?;

    println!("8:1 - {}", run_1(&input)?);
    println!("8:2 - {}", run_2(&input)?);

    Ok(())
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let mut antennas: HashMap<char, Vec<Coord>> = HashMap::new();
    let mut max_row = 0;
    let mut max_col = 0;
    for (row, line) in input.lines().enumerate() {
        max_row = max_row.max(row);
        for (col, ch) in line.chars().enumerate() {
            max_col = max_col.max(col);
            match ch {
                '.' => (),
                a => {
                    let e = antennas.entry(a).or_default();
                    e.push((row, col).into());
                }
            }
        }
    }
    let mut antinodes: HashSet<Coord> = HashSet::new();
    for a in antennas.values() {
        for (idx, a1) in a.iter().enumerate() {
            for a2 in a.iter().skip(idx + 1) {
                let dx = a2.icol() - a1.icol();
                let dy = a2.irow() - a1.irow();
                antinodes.insert(*a1 - (dy, dx).into());
                antinodes.insert(*a2 + (dy, dx).into());
            }
        }
    }
    Ok(antinodes
        .iter()
        .filter(|c| {
            c.irow() >= 0
                && c.irow() <= max_row as isize
                && c.icol() >= 0
                && c.icol() <= max_col as isize
        })
        .count())
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let mut antennas: HashMap<char, Vec<Coord>> = HashMap::new();
    let mut max_row = 0;
    let mut max_col = 0;
    for (row, line) in input.lines().enumerate() {
        max_row = max_row.max(row);
        for (col, ch) in line.chars().enumerate() {
            max_col = max_col.max(col);
            match ch {
                '.' => (),
                a => {
                    let e = antennas.entry(a).or_default();
                    e.push((row, col).into());
                }
            }
        }
    }
    let mut antinodes: HashSet<Coord> = HashSet::new();
    for a in antennas.values() {
        for (idx, a1) in a.iter().enumerate() {
            for a2 in a.iter().skip(idx + 1) {
                let dx = a2.icol() - a1.icol();
                let dy = a2.irow() - a1.irow();
                let d:Coord = (dy,dx).into();
                for i in 0..50 {
                    antinodes.insert(*a1 - i * d);
                    antinodes.insert(*a2 + i * d);
                }
            }
        }
    }
    Ok(antinodes
        .iter()
        .filter(|c| {
            c.irow() >= 0
                && c.irow() <= max_row as isize
                && c.icol() >= 0
                && c.icol() <= max_col as isize
        })
        .count())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    const INPUT_A: &str = "..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
..........";

    #[test]
    fn day8_run_1() {
        assert_eq!(super::run_1(INPUT_A).unwrap(), 2);
        assert_eq!(super::run_1(INPUT).unwrap(), 14);
        //
    }

    const INPUT_T: &str = "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........";

    #[test]
    fn day8_run_2() {
        assert_eq!(super::run_2(INPUT_T).unwrap(), 9);
        assert_eq!(super::run_2(INPUT).unwrap(), 34);
        //
    }
}
