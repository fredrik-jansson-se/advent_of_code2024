use nom::Parser;

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day24.txt")?;

    println!("24:1 - {}", run_1(&input)?);
    println!("24:2 - {}", run_2(&input)?);

    Ok(())
}

#[derive(Debug)]
enum Op {
    Or,
    Xor,
    And,
}

fn parse_op(i: crate::Input) -> crate::PResult<Op> {
    let or = nom::combinator::map(nom::bytes::complete::tag("OR"), |_| Op::Or);
    let xor = nom::combinator::map(nom::bytes::complete::tag("XOR"), |_| Op::Xor);
    let and = nom::combinator::map(nom::bytes::complete::tag("AND"), |_| Op::And);
    nom::branch::alt((or, xor, and)).parse(i)
}

struct Assignment<'a> {
    name: &'a str,
    value: u8,
}

fn parse_assign(i: crate::Input) -> crate::PResult<Assignment> {
    let (i, name) = nom::character::complete::alphanumeric1(i)?;
    let (i, _) = nom::bytes::complete::tag(":").parse(i)?;
    let (i, _) = nom::character::complete::space1(i)?;
    let (i, value) = nom::character::complete::u8(i)?;
    Ok((i, Assignment { name, value }))
}

#[derive(Debug)]
struct Gate {
    input_1: String,
    input_2: String,
    output: String,
    op: Op,
}

fn parse_gate(i: crate::Input) -> crate::PResult<Gate> {
    let (i, input_1) = nom::character::complete::alphanumeric1(i)?;
    let (i, _) = nom::character::complete::space1(i)?;
    let (i, op) = parse_op(i)?;
    let (i, _) = nom::character::complete::space1(i)?;
    let (i, input_2) = nom::character::complete::alphanumeric1(i)?;
    let (i, _) = nom::bytes::complete::tag(" -> ").parse(i)?;
    let (i, output) = nom::character::complete::alphanumeric1(i)?;
    Ok((
        i,
        Gate {
            input_1: input_1.to_owned(),
            input_2: input_2.to_owned(),
            output: output.to_owned(),
            op,
        },
    ))
}

#[derive(Debug)]
struct Program {
    wires: std::collections::HashMap<String, u8>,
    gates: Vec<Gate>,
}

fn parse(i: crate::Input) -> crate::PResult<Program> {
    let (i, assignments) =
        nom::multi::separated_list1(nom::character::complete::newline, parse_assign).parse(i)?;
    let (i, _) = nom::character::complete::newline(i)?;
    let (i, _) = nom::character::complete::newline(i)?;
    let (i, gates) =
        nom::multi::separated_list1(nom::character::complete::newline, parse_gate).parse(i)?;

    let mut wires = std::collections::HashMap::new();
    for a in assignments {
        wires.insert(a.name.to_owned(), a.value);
    }

    Ok((i, Program { wires, gates }))
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let (_, mut program) = parse(input).map_err(|e| e.to_owned())?;
    while !program.gates.is_empty() {
        let mut idx = 0;
        while idx < program.gates.len() {
            let gate = &mut program.gates[idx];
            let Some(input_1) = program.wires.get(&gate.input_1) else {
                idx += 1;
                continue;
            };
            let Some(input_2) = program.wires.get(&gate.input_2) else {
                idx += 1;
                continue;
            };

            let output = match gate.op {
                Op::Or => input_1 | input_2,
                Op::Xor => input_1 ^ input_2,
                Op::And => input_1 & input_2,
            };

            program.wires.insert(gate.output.clone(), output);

            program.gates.remove(idx);
        }
    }

    let mut res: usize = 0;
    for zidx in (0..99).rev() {
        if let Some(v) = program.wires.get(&format!("z{zidx:02}")) {
            res <<= 1;
            res += *v as usize;
        }
    }
    Ok(res)
}

fn run_2(_input: &str) -> anyhow::Result<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    const INPUT_1: &str = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";

    const INPUT_2: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    #[test]
    fn day24_run_1() {
        assert_eq!(super::run_1(INPUT_1).unwrap(), 4);
        assert_eq!(super::run_1(INPUT_2).unwrap(), 2024);
    }

    #[test]
    fn day24_run_2() {}
}
