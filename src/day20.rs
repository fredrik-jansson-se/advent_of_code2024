use crate::common::Coord;

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day20.txt")?;

    println!("20:1 - {}", run_1(&input, 100)?);
    println!("20:2 - {}", run_2(&input)?);

    Ok(())
}

#[derive(Default)]
struct Map {
    start: Coord,
    end: Coord,
    walls: std::collections::HashSet<Coord>,
}

fn parse_map(i: &str) -> Map {
    let mut map: Map = Default::default();

    for (r, row) in i.lines().enumerate() {
        for (c, ch) in row.chars().enumerate() {
            match ch {
                '#' => {
                    map.walls.insert((r, c).into());
                }
                'S' => {
                    map.start = (r, c).into();
                }
                'E' => {
                    map.end = (r, c).into();
                }
                '.' => (),
                _ => unreachable!(),
            }
        }
    }

    map
}

#[derive(Clone, Debug)]
struct State {
    can_cheat: bool,
    pos: Coord,
}

impl std::cmp::PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl std::hash::Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
    }
}

impl std::cmp::Eq for State {}

fn find_all_routes(map: &Map, min_save: usize) -> usize {
    // start without cheat to find the non cheating path
    let start_state = State {
        can_cheat: false,
        pos: map.start,
    };

    let (res, non_cheat_len) = pathfinding::directed::astar::astar(
        &start_state,
        |s| {
            s.pos
                .neighbors()
                .filter(|n| !map.walls.contains(n))
                .map(|n| {
                    (
                        State {
                            can_cheat: false,
                            pos: n,
                        },
                        1usize,
                    )
                })
        },
        |s| s.pos.manhattan(&map.end),
        |s| s.pos == map.end,
    )
    .expect("Failed to find path");

    // let non_cheat_len = res.len();
    println!("Non cheat len: {non_cheat_len}");

    // Now allow cheats
    let start_state = State {
        can_cheat: true,
        pos: map.start,
    };

    let mut lookup: std::collections::HashMap<usize, usize> = Default::default();
    for can_cheat_at in res.iter().map(|s| s.pos) {
        if let Some((_path, len)) = pathfinding::directed::astar::astar(
            &start_state,
            |s| {
                let can_cheat = s.can_cheat;
                let cheat_nbrs: Vec<(State, usize)> = if can_cheat && s.pos == can_cheat_at {
                    s.pos
                        .neighbors_with_step(2)
                        .filter(|p| !map.walls.contains(p))
                        .map(|n| {
                            (
                                State {
                                    can_cheat: false,
                                    pos: n,
                                },
                                2usize,
                            )
                        })
                        .collect()
                } else {
                    Vec::new()
                };
                let cheat_nbrs = cheat_nbrs.into_iter();
                s.pos
                    .neighbors()
                    .filter(|n| !map.walls.contains(n))
                    .map(move |n| (State { can_cheat, pos: n }, 1usize))
                    .chain(cheat_nbrs)
            },
            |s| s.pos.manhattan(&map.end),
            |s| s.pos == map.end,
        ) {
            if len < non_cheat_len {
                let d = non_cheat_len - len;
                if d >= min_save {
                    *lookup.entry(d).or_default() += 1;
                    println!("{len} -> {}", non_cheat_len - len);
                }
            }
        }
    }
    dbg! {&lookup};
    // for r in results.iter().filter(|r| r.len() < non_cheat_len) {
    //     println!("d: {}", non_cheat_len - r.len());
    // }

    lookup.values().sum()
}

fn run_1(input: &str, _least_save: usize) -> anyhow::Result<usize> {
    let map = parse_map(input);
    let res = find_all_routes(&map, 100);
    Ok(res)
}

fn run_2(_input: &str) -> anyhow::Result<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    const INPUT_1: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    #[ignore]
    fn day20_run_1() {
        assert_eq!(super::run_1(INPUT_1, 1).unwrap(), 44);
    }

    #[test]
    fn day20_run_2() {}
}
