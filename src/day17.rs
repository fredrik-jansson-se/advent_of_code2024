pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day17.txt")?;

    println!("17:1 - {}", run_1(&input)?);
    println!("17:2 - {}", run_2(&input)?);

    Ok(())
}

fn run_1(_input: &str) -> anyhow::Result<usize> {
  todo!()
}

fn run_2(_input: &str) -> anyhow::Result<usize> {
  todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn day17_run_1() {
    }

    #[test]
    fn day17_run_2() {
    }
}
