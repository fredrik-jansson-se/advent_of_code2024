use nom::Parser;

use crate::{
    common::{Coord, Dir},
    Input, PResult,
};

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day15.txt")?;

    println!("15:1 - {}", run_1(&input)?);
    println!("15:2 - {}", run_2(&input)?);

    Ok(())
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum MapItem {
    Wall,
    Box,
    Robot,
    Empty,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum MapItem2 {
    Wall,
    Box,
    Empty,
}

type Map = Vec<Vec<MapItem>>;
type Map2 = Vec<Vec<MapItem2>>;
type Moves = Vec<Dir>;

fn parse_map(i: Input) -> PResult<Map> {
    use nom::bytes::complete::tag;
    use nom::combinator::map;
    let wall = map(tag("#"), |_| MapItem::Wall);
    let r#box = map(tag("O"), |_| MapItem::Box);
    let robot = map(tag("@"), |_| MapItem::Robot);
    let empty = map(tag("."), |_| MapItem::Empty);

    let row = nom::multi::many1(nom::branch::alt((wall, r#box, robot, empty)));

    nom::multi::separated_list1(nom::character::complete::newline, row).parse(i)
}

fn parse(i: Input) -> PResult<(Map, Moves)> {
    use nom::bytes::complete::tag;
    use nom::character::complete::newline;
    use nom::combinator::map;
    let (i, the_map) = parse_map(i)?;
    let (i, _) = newline(i)?;
    let (i, _) = newline(i)?;

    let up = map(
        nom::sequence::terminated(tag("^"), nom::combinator::opt(newline)),
        |_| Dir::N,
    );

    let left = map(
        nom::sequence::terminated(tag("<"), nom::combinator::opt(newline)),
        |_| Dir::W,
    );
    let down = map(
        nom::sequence::terminated(tag("v"), nom::combinator::opt(newline)),
        |_| Dir::S,
    );
    let right = map(
        nom::sequence::terminated(tag(">"), nom::combinator::opt(newline)),
        |_| Dir::E,
    );

    let (i, moves) = nom::multi::many1(nom::branch::alt((up, left, down, right))).parse(i)?;

    Ok((i, (the_map, moves)))
}

fn move_boxes(box_pos: Coord, dir: Dir, map: &mut Map) -> bool {
    let move_to_pos = box_pos + dir.movement();
    match map
        .get(move_to_pos.row())
        .and_then(|line| line.get(move_to_pos.col()))
    {
        None => unreachable!(),
        Some(MapItem::Wall) => false,
        Some(MapItem::Box) => {
            if move_boxes(move_to_pos, dir, map) {
                map[box_pos.row()][box_pos.col()] = MapItem::Empty;
                map[move_to_pos.row()][move_to_pos.col()] = MapItem::Box;
                true
            } else {
                false
            }
        }
        Some(MapItem::Robot) => {
            map[box_pos.row()][box_pos.col()] = MapItem::Empty;
            map[move_to_pos.row()][move_to_pos.col()] = MapItem::Box;
            true
        }
        Some(MapItem::Empty) => {
            map[box_pos.row()][box_pos.col()] = MapItem::Empty;
            map[move_to_pos.row()][move_to_pos.col()] = MapItem::Box;
            true
        }
    }
}

// Will alway have the boxes left position
fn move_boxes2(box_pos: Coord, dir: Dir, map: &mut Map2, do_move: bool) -> bool {
    let horizontal = dir == Dir::E || dir == Dir::W;
    let move_to_pos = if horizontal {
        box_pos + dir.movement() + dir.movement()
    } else {
        box_pos + dir.movement()
    };

    let item = map[move_to_pos.row()][move_to_pos.col()];
    let to_left = move_to_pos + Dir::W.movement();
    let in_box = item == MapItem2::Box || map[to_left.row()][to_left.col()] == MapItem2::Box;

    if MapItem2::Wall == item {
        return false;
    } else if MapItem2::Empty == item && !in_box {
        if do_move {
            let move_to_pos = move_to_pos - dir.movement();
            map[move_to_pos.row()][move_to_pos.col()] = MapItem2::Box;
        }
        return true;
    }

    // We know we're in a box now

    let left_pos = match item {
        MapItem2::Box => move_to_pos,
        MapItem2::Empty => move_to_pos + Dir::W.movement(),
        _ => unreachable!()
    };
    
    let _can_move = move_boxes2(left_pos, dir, map, do_move);

    //if do_move && can_move {
    //    if horizontal {
    //    }
    //    else {
    //
    //    }
    //}

    //match map
    //    .get(move_to_pos.row())
    //    .and_then(|line| line.get(move_to_pos.col()))
    //{
    //    None => unreachable!(),
    //    Some(MapItem2::Empty) => {
    //        if do_move {
    //            map[box_pos.row()][box_pos.col()] = MapItem::Empty;
    //            if dir == Dir::E {
    //                map[move_to_pos.row()][move_to_pos.col()] = MapItem2::BoxRight;
    //                let move_to_pos = move_to_pos - dir.movement();
    //                map[move_to_pos.row()][move_to_pos.col()] = MapItem2::BoxLeft;
    //            }
    //            else if dir == Dir::W {
    //                map[move_to_pos.row()][move_to_pos.col()] = MapItem2::BoxLeft;
    //                let move_to_pos = move_to_pos - dir.movement();
    //                map[move_to_pos.row()][move_to_pos.col()] = MapItem2::BoxRight;
    //            }
    //        }
    //        return true;
    //    }
    //    Some(MapItem2::BoxLeft) if dir == Dir::N => {
    //        let box_right_pos = move_to_pos + (0, 1).into();
    //        if move_boxes2(move_to_pos, dir, map, do_move)
    //            && move_boxes2(box_right_pos, dir, map, do_move)
    //        {
    //            if do_move {
    //                //map[box_pos.row()][box_pos.col()] = MapItem::Empty;
    //                //map[move_to_pos.row()][move_to_pos.col()] = MapItem::Box;
    //            }
    //            true
    //        } else {
    //            false
    //        }
    //    }
    //    Some(MapItem2::BoxRight) => {
    //        todo!()
    //    }
    //};

    todo!()
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let (i, (mut map, moves)) = parse(input).map_err(|e| anyhow::anyhow!("{e}"))?;
    assert!(i.len() <= 1);

    let mut robot_pos: Coord = map
        .iter()
        .enumerate()
        .find_map(|(row, line)| {
            line.iter().enumerate().find_map(|(col, i)| {
                if *i == MapItem::Robot {
                    Some((row, col).into())
                } else {
                    None
                }
            })
        })
        .unwrap();

    for m in moves {
        let move_to_pos = robot_pos + m.movement();
        match map
            .get(move_to_pos.row())
            .and_then(|line| line.get(move_to_pos.col()))
        {
            None => unreachable!(),
            Some(MapItem::Wall) => (),
            Some(MapItem::Box) => {
                if move_boxes(move_to_pos, m, &mut map) {
                    map[robot_pos.row()][robot_pos.col()] = MapItem::Empty;
                    map[move_to_pos.row()][move_to_pos.col()] = MapItem::Robot;
                    robot_pos = move_to_pos;
                }
            }
            Some(MapItem::Robot) => {
                robot_pos = move_to_pos;
            }
            Some(MapItem::Empty) => {
                robot_pos = move_to_pos;
            }
        }
    }

    let mut sum = 0;
    for (row, line) in map.iter().enumerate() {
        for (col, i) in line.iter().enumerate() {
            if *i == MapItem::Box {
                sum += 100 * row + col;
            }
        }
    }

    Ok(sum)
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let (i, ( old_map, moves)) = parse(input).map_err(|e| anyhow::anyhow!("{e}"))?;
    assert!(i.len() <= 1);

    let mut robot_pos: Coord = (0, 0).into();
    let mut map = Vec::new();
    for (row, line) in old_map.iter().enumerate() {
        let mut new_line = Vec::new();
        for (col, i) in line.iter().enumerate() {
            match i {
                MapItem::Wall => {
                    new_line.push(MapItem2::Wall);
                    new_line.push(MapItem2::Wall);
                }
                MapItem::Box => {
                    new_line.push(MapItem2::Box);
                    new_line.push(MapItem2::Empty);
                }
                MapItem::Robot => {
                    robot_pos = (row, col).into();
                    new_line.push(MapItem2::Empty);
                    new_line.push(MapItem2::Empty);
                }
                MapItem::Empty => {
                    new_line.push(MapItem2::Empty);
                    new_line.push(MapItem2::Empty);
                }
            }
        }
        map.push(new_line);
    }

    for m in moves {
        let move_to_pos = robot_pos + m.movement();
        let to_left = move_to_pos + Dir::W.movement();
        let item = map[move_to_pos.row()][move_to_pos.col()];
        let left_is_box = map[to_left.row()][to_left.col()] == MapItem2::Box;
        match item
        {
            MapItem2::Wall => (),
            MapItem2::Empty if !left_is_box => {
                robot_pos = move_to_pos;
            },
            _box => {
                let box_pos = if left_is_box {
                    move_to_pos + Dir::W.movement()
                } else {
                    move_to_pos
                };
                if move_boxes2(box_pos, m, &mut map, false) {
                    move_boxes2(box_pos, m, &mut map, true);
                    map[robot_pos.row()][robot_pos.col()] = MapItem2::Empty;
                    map[move_to_pos.row()][move_to_pos.col()] = MapItem2::Empty;
                    robot_pos = move_to_pos;
                }
            }
        }
    }

    let mut sum = 0;
    for (row, line) in map.iter().enumerate() {
        for (col, i) in line.iter().enumerate() {
            if *i == MapItem2::Box {
                sum += 100 * row + col;
            }
        }
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    const INPUT_1: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
    #[test]
    fn day15_run_1() {
        assert_eq!(super::run_1(INPUT_1).unwrap(), 10092);
    }

    #[test]
    #[ignore]
    fn day15_run_2() {
        assert_eq!(super::run_2(INPUT_1).unwrap(), 9021);
    }
}
