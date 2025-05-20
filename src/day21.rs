pub fn run() -> anyhow::Result<()> {
    let input = "593A
283A
670A
459A
279A";

    println!("21:1 - {}", run_1(input)?);
    println!("21:2 - {}", run_2(input)?);

    Ok(())
}

/*
+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+
*/
// struct Keypad {
//     key_coord: std::collections::HashMap<char, Coord>,
//     pos: char,
// }

// impl Keypad {
//     fn new() -> Self {
//         let key_coord = [
//             ('7', (0, 0).into()),
//             ('8', (0, 1).into()),
//             ('9', (0, 2).into()),
//             ('4', (1, 0).into()),
//             ('5', (1, 1).into()),
//             ('6', (1, 2).into()),
//             ('1', (2, 0).into()),
//             ('2', (2, 1).into()),
//             ('3', (2, 2).into()),
//             ('0', (3, 1).into()),
//             ('A', (3, 2).into()),
//         ]
//         .into_iter()
//         .collect();
//         Self {
//             key_coord,
//             pos: 'A',
//         }
//     }
//
//     fn move_to(&mut self, target: char) -> Vec<char> {
//         let start_pos = self.key_coord[&self.pos];
//         let end_pos = self.key_coord[&target];
//         let valid: std::collections::HashSet<&Coord> = self.key_coord.values().collect();
//         let (path, _) = pathfinding::directed::astar::astar(
//             &start_pos,
//             |p: &Coord| {
//                 let nbrs = p.neighbors();
//                 nbrs.filter(|n| valid.contains(n))
//                     .map(|n| (n, 1))
//                     .into_iter()
//             },
//             |p| p.manhattan(&end_pos),
//             |p| *p == end_pos,
//         )
//         .expect("Find a keypad path");
//
//         path.chunks(2).map(|_c| {
//             // match c[1] - c[0] {
//             //     Coord::new(0,1) => '>',
//             //     Coord::new(0, -1) => '<',
//             //     // Coord::new(
//             // }
//             'a'
//         }).collect()
//     }
// }

fn run_1(_input: &str) -> anyhow::Result<usize> {
    todo!()
}

fn run_2(_input: &str) -> anyhow::Result<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "029A
980A
179A
456A
379A";

    #[test]
    #[ignore]
    fn day21_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 126384);
    }

    #[test]
    fn day21_run_2() {}

    // #[test]
    // fn day21_keypad() {
    //     let mut k = super::Keypad::new();
    //     let s1 = k.move_to('0');
    // }
}
