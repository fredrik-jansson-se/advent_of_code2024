pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day22.txt")?;

    println!("22:1 - {}", run_1(&input)?);
    println!("22:2 - {}", run_2(&input)?);

    Ok(())
}

fn calc_secret(secret: usize) -> usize {
    const M: usize = 16777216;
    let secret = ((secret * 64) ^ secret) % M;
    let secret = ((secret / 32) ^ secret) % M;
    ((secret * 2048) ^ secret) % M
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let nums: Vec<usize> = input.lines().map(|v| v.parse().unwrap()).collect();
    let sum = nums
        .into_iter()
        .map(|n| (0..2000).fold(n, |n, _| calc_secret(n)))
        .sum();
    Ok(sum)
}

fn run_2(_input: &str) -> anyhow::Result<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "1
10
100
2024";

    #[test]
    fn day22_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 37327623);
    }

    #[test]
    fn day22_run_2() {}
}
