pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day9.txt")?;

    println!("9:1 - {}", run_1(&input)?);
    println!("9:2 - {}", run_2(&input)?);

    Ok(())
}

type Disk = Vec<Option<usize>>;

fn parse(input: &str) -> Disk {
    let mut disk = Vec::new();
    for (idx, ch) in input.chars().enumerate() {
        let Some(num) = ch.to_digit(10) else {
            break;
        };
        let val = if idx % 2 == 0 { Some(idx / 2) } else { None };
        for _ in 0..num {
            disk.push(val);
        }
    }
    disk
}

fn compact(mut disk: Disk) -> Disk {
    let mut back_idx = disk.len() - 1;
    'outer: for idx in 0..disk.len() {
        if disk[idx].is_none() {
            while disk[back_idx].is_none() {
                if back_idx <= idx {
                    break 'outer;
                }
                back_idx -= 1;
            }
            disk.swap(idx, back_idx);
        }
    }
    disk
}

fn find_free_space(disk: &[Option<usize>], needed: usize) -> Option<usize> {
    let mut idx = 0;
    while idx < disk.len() {
        idx += disk.iter().skip(idx).take_while(|p| p.is_some()).count();
        let free = disk.iter().skip(idx).take_while(|p| p.is_none()).count();
        //println!("Found free: {free}");
        if needed <= free {
            return Some(idx);
        }
        idx += free;
    }
    None
}

fn compact_2(mut disk: Disk) -> Disk {
    let mut back_idx = disk.len() - 1;

    while back_idx > 0 {
        while back_idx > 0 && disk[back_idx].is_none() {
            back_idx -= 1;
        }
        let Some(id) = disk[back_idx] else {
            unreachable!();
        };
        let mut start_idx = back_idx - 1;
        while Some(id) == disk[start_idx] && start_idx > 0 {
            start_idx -= 1;
        }
        // Last we found was bad
        start_idx += 1;

        let l = back_idx - start_idx + 1;
        //println!("Looking for {id}: {l}");
        if let Some(idx) = find_free_space(&disk[..start_idx], l) {
            //println!("moving {id} -> {idx}");
            for i in 0..l {
                disk.swap(idx + i, start_idx + i);
            }
        }
        back_idx = start_idx-1;
    }
    disk
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let disk = parse(input);
    let disk = compact(disk);
    Ok(disk
        .iter()
        .enumerate()
        .filter_map(|(idx, f)| f.map(|f| f * idx))
        .sum())
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let disk = parse(input);
    let disk = compact_2(disk);
    Ok(disk
        .iter()
        .enumerate()
        .filter_map(|(idx, f)| f.map(|f| f * idx))
        .sum())
}

#[cfg(test)]
mod tests {
    const INPUT_1: &str = "2333133121414131402";
    const INPUT_2: &str = "12345";

    #[test]
    fn day9_parse() {
        let disk = super::parse(INPUT_2);
        assert_eq!(
            disk,
            vec![
                Some(0),
                None,
                None,
                Some(1),
                Some(1),
                Some(1),
                None,
                None,
                None,
                None,
                Some(2),
                Some(2),
                Some(2),
                Some(2),
                Some(2)
            ]
        );
    }
    #[test]
    fn day9_compact() {
        let disk = super::parse(INPUT_2);
        assert_eq!(
            super::compact(disk),
            vec![
                Some(0),
                Some(2),
                Some(2),
                Some(1),
                Some(1),
                Some(1),
                Some(2),
                Some(2),
                Some(2),
                None,
                None,
                None,
                None,
                None,
                None,
            ]
        );
    }

    #[test]
    fn day9_run_1() {
        assert_eq!(super::run_1(INPUT_1).unwrap(), 1928);
    }

    #[test]
    fn day9_run_2() {
        assert_eq!(super::run_2(INPUT_1).unwrap(), 2858);
    }
}
