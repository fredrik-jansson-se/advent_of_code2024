use std::collections::{HashMap, HashSet};

use crate::common::Coord;

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day12.txt")?;

    println!("12:1 - {}", run_1(&input)?);
    println!("12:2 - {}", run_2(&input)?);

    Ok(())
}

type Map = Vec<Vec<char>>;

fn run_1(input: &str) -> anyhow::Result<usize> {
    let mut map: Map = Vec::new();
    let mut to_check: HashSet<Coord> = HashSet::new();
    for (row, line) in input.lines().enumerate() {
        let mut v = Vec::new();
        for (col, ch) in line.chars().enumerate() {
            v.push(ch);
            to_check.insert((row, col).into());
        }
        map.push(v);
    }
    let max_col = map[0].len();
    let max_row = map.len();

    let mut nbr_cnt = HashMap::new();
    let mut res = 0;
    while let Some(c) = to_check.iter().next().cloned() {
        nbr_cnt.clear();
        to_check.remove(&c);
        let name = map[c.row()][c.col()];

        // Find all nbrs of the same name
        let mut nbrs: Vec<_> = vec![c];
        nbr_cnt.insert(c, nbrs.len());

        // go thorugh all nbrs
        while let Some(n) = nbrs.pop() {
            let new_nbrs: Vec<_> = n
                .neighbors()
                .filter(Coord::is_positive)
                .filter(|c| c.row() < max_row && c.col() < max_col)
                .filter(|c| name == map[c.row()][c.col()])
                .collect();

            for n in &new_nbrs {
                to_check.remove(n);
            }

            nbr_cnt.insert(n, new_nbrs.len());
            nbrs.extend(new_nbrs.into_iter().filter(|c| !nbr_cnt.contains_key(c)));
        }

        let area = nbr_cnt.len();
        let mut len = 0;

        for (_, v) in nbr_cnt.drain() {
            len += 4 - v;
        }

        res += area * len;
    }

    Ok(res)
}

fn run_2(_input: &str) -> anyhow::Result<usize> {
    todo!()
    //let mut map: Map = Vec::new();
    //let mut to_check: HashSet<Coord> = HashSet::new();
    //for (row, line) in input.lines().enumerate() {
    //    let mut v = Vec::new();
    //    for (col, ch) in line.chars().enumerate() {
    //        v.push(ch);
    //        to_check.insert((row, col).into());
    //    }
    //    map.push(v);
    //}
    //let max_col = map[0].len();
    //let max_row = map.len();
    //
    //let mut nbr_cnt = HashMap::new();
    //let mut res = 0;
    //while let Some(c) = to_check.iter().next().cloned() {
    //    nbr_cnt.clear();
    //    to_check.remove(&c);
    //    let name = map[c.row()][c.col()];
    //
    //    // Find all nbrs of the same name
    //    let mut nbrs: Vec<_> = vec![c];
    //    nbr_cnt.insert(c, nbrs.len());
    //
    //    // go thorugh all nbrs
    //    while let Some(n) = nbrs.pop() {
    //        let new_nbrs: Vec<_> = n
    //            .neighbors()
    //            .filter(Coord::is_positive)
    //            .filter(|c| c.row() < max_row && c.col() < max_col)
    //            .filter(|c| name == map[c.row()][c.col()])
    //            .collect();
    //
    //        for n in &new_nbrs {
    //            to_check.remove(n);
    //        }
    //
    //        nbr_cnt.insert(n, new_nbrs.len());
    //        nbrs.extend(new_nbrs.into_iter().filter(|c| !nbr_cnt.contains_key(c)));
    //    }
    //
    //    let area = nbr_cnt.len();
    //
    //    let mut edge_nbrs: HashSet<Coord> = nbr_cnt
    //        .iter()
    //        .filter(|(_, cnt)| **cnt < 4)
    //        .map(|(c, _)| *c)
    //        .collect();
    //
    //    //let mut cur_dir = Dir::E;
    //    let len = 0;
    //    loop {
    //        let Some(start_nbr) = edge_nbrs.iter().min().cloned() else {
    //            break;
    //        };
    //        loop {
    //            edge_nbrs.remove(&start_nbr);
    //            //let nbr = start_nbr + cur_dir.movement();
    //        }
    //    }
    //
    //    res += area * len;
    //    break;
    //}
    //Ok(res)
}

#[cfg(test)]
mod tests {
    const INPUT_1: &str = "AAAA
BBCD
BBCC
EEEC";

    const INPUT_2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    const INPUT_3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    const INPUT_4: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

    const INPUT_5: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

    const INPUT_SINGLE: &str = "AAAA";

    #[test]
    fn day12_run_1() {
        assert_eq!(super::run_1(INPUT_1).unwrap(), 140);
        assert_eq!(super::run_1(INPUT_2).unwrap(), 772);
        assert_eq!(super::run_1(INPUT_3).unwrap(), 1930);
    }

    #[test]
    #[ignore]
    fn day12_run_2() {
        assert_eq!(super::run_2(INPUT_SINGLE).unwrap(), 16);
        assert_eq!(super::run_2(INPUT_1).unwrap(), 80);
        assert_eq!(super::run_2(INPUT_2).unwrap(), 436);
        assert_eq!(super::run_2(INPUT_4).unwrap(), 236);
        assert_eq!(super::run_2(INPUT_5).unwrap(), 368);
        assert_eq!(super::run_2(INPUT_3).unwrap(), 1206);
    }
}
