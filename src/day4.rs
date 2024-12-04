pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day4.txt")?;

    println!("4:1 - {}", run_1(&input)?);
    println!("4:2 - {}", run_2(&input)?);

    Ok(())
}

fn get_char(words: &[Vec<char>], row: usize, col: isize) -> char {
    if col < 0 || col >= words[0].len() as isize {
        return '.';
    }
    words[row][col as usize]
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let words: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut num_xmas = 0;
    let xmas_f = &['X', 'M', 'A', 'S'];
    let xmas_r = &['S', 'A', 'M', 'X'];
    let max_row = words.len();
    let max_col = words[0].len();
    let down = &mut ['a'; 4];
    let down_left = &mut ['a'; 4];
    let down_right = &mut ['a'; 4];

    for row in 0..max_row {
        // Find in current row
        //print!("{row}: {num_xmas} -> ");
        num_xmas += words[row]
            .windows(4)
            //.inspect(|s| {
            //    dbg! {s};
            //})
            .filter(|s| s == xmas_r || s == xmas_f)
            .count();
        //println!("{num_xmas}");

        if row <= (max_row - 4) {
            for col in 0..max_col {
                if !xmas_f.contains(&words[row][col]) {
                    continue;
                }

                for dr in 0..4 {
                    down[dr] = get_char(&words, row + dr, col as _);
                    down_left[dr] = get_char(&words, row + dr, (col as isize) - dr as isize);
                    down_right[dr] = get_char(&words, row + dr, (col as isize) + dr as isize);
                }

                if down == xmas_f || down == xmas_r {
                    num_xmas += 1;
                }
                if down_left == xmas_f || down_left == xmas_r {
                    num_xmas += 1;
                }
                if down_right == xmas_f || down_right == xmas_r {
                    num_xmas += 1;
                }
            }
        }
    }

    Ok(num_xmas)
}
/*
 *
 */

fn run_2(input: &str) -> anyhow::Result<usize> {
    let words: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mas_f = &['M', 'A', 'S'];
    let mas_r = &['S', 'A', 'M'];
    let down_left = &mut ['a'; 3];
    let down_right = &mut ['a'; 3];
    let max_row = words.len();
    let max_col = words[0].len();
    let mut num_mas = 0;
    for row in 0..(max_row - 2) {
        for col in 0..(max_col - 2) {
            if !mas_f.contains(&words[row][col]) {
                continue;
            }
            for dr in 0..3 {
                down_left[dr] = get_char(&words, row + dr, 2 + (col as isize) - dr as isize);
                down_right[dr] = get_char(&words, row + dr, (col as isize) + dr as isize);
            }

            if (down_left == mas_f || down_left == mas_r)
                && (down_right == mas_f || down_right == mas_r)
            {
                num_mas += 1;
            }
        }
    }

    Ok(num_mas)
}

#[cfg(test)]
mod tests {
    const INPUT_1: &str = "..X...
.SAMX.
.A..A.
XMAS.S
.X....";

    const INPUT_2: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    const INPUT_3: &str = "S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.";

    #[test]
    fn day4_run_1() {
        assert_eq!(super::run_1(INPUT_1).unwrap(), 4);
        assert_eq!(super::run_1(INPUT_2).unwrap(), 18);
    }

    #[test]
    fn day4_run_2() {
        assert_eq!(super::run_2(INPUT_3).unwrap(), 4);
        assert_eq!(super::run_2(INPUT_2).unwrap(), 9);
    }
}
