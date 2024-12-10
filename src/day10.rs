use crate::common::Coord;

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day10.txt")?;

    println!("10:1 - {}", run_1(&input)?);
    println!("10:2 - {}", run_2(&input)?);

    Ok(())
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let map: Vec<Vec<isize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap_or(10) as _)
                .collect()
        })
        .collect();

    let mut trailheads: Vec<Coord> = Vec::new();
    let mut trailends: Vec<Coord> = Vec::new();
    for (row, line) in map.iter().enumerate() {
        for (col, h) in line.iter().enumerate() {
            if *h == 0 {
                trailheads.push((row, col).into());
            } else if *h == 9 {
                trailends.push((row, col).into());
            }
        }
    }

    let mut score = 0;
    let mb = &map;
    for th in &trailheads {
        for te in &trailends {
            let path = pathfinding::directed::astar::astar(
                th,
                |c: &Coord| {
                    let cur_height = map[c.row()][c.col()];
                    c.neighbors()
                        .filter(|c| c.irow() >= 0 && c.icol() >= 0)
                        .filter(move |c| {
                            mb.get(c.row())
                                .and_then(|r| r.get(c.col()))
                                .cloned()
                                .unwrap_or(100)
                                - cur_height
                                == 1
                        })
                        .map(|c| (c, 1))
                },
                |c: &Coord| c.manhattan(te),
                |c: &Coord| c == te,
            );
            if path.is_some() {
                score += 1;
            }
        }
    }

    Ok(score)
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let map: Vec<Vec<isize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap_or(10) as _)
                .collect()
        })
        .collect();

    let mut trailheads: Vec<Coord> = Vec::new();
    let mut trailends: Vec<Coord> = Vec::new();
    for (row, line) in map.iter().enumerate() {
        for (col, h) in line.iter().enumerate() {
            if *h == 0 {
                trailheads.push((row, col).into());
            } else if *h == 9 {
                trailends.push((row, col).into());
            }
        }
    }

    let mut score = 0;
    let mb = &map;
    for th in &trailheads {
        for te in &trailends {
            let path = pathfinding::directed::astar::astar_bag_collect(
                th,
                |c: &Coord| {
                    let cur_height = map[c.row()][c.col()];
                    c.neighbors()
                        .filter(|c| c.irow() >= 0 && c.icol() >= 0)
                        .filter(move |c| {
                            mb.get(c.row())
                                .and_then(|r| r.get(c.col()))
                                .cloned()
                                .unwrap_or(100)
                                - cur_height
                                == 1
                        })
                        .map(|c| (c, 1))
                },
                |c: &Coord| c.manhattan(te),
                |c: &Coord| c == te,
            );
            score += path.map(|p| p.0.len()).unwrap_or(0);
        }
    }

    Ok(score)
}

#[cfg(test)]
mod tests {
    const INPUT_1: &str = "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9";

    const INPUT_2: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn day10_run_1() {
        assert_eq!(super::run_1(INPUT_1).unwrap(), 2);
        assert_eq!(super::run_1(INPUT_2).unwrap(), 36);
    }

    #[test]
    fn day10_run_2() {
        assert_eq!(super::run_2(INPUT_2).unwrap(), 81);
    }
}
