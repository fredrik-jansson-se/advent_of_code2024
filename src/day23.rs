use std::collections::HashMap;

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day23.txt")?;

    println!("23:1 - {}", run_1(&input)?);
    println!("23:2 - {}", run_2(&input)?);

    Ok(())
}

fn build_graph(
    i: &str,
) -> (
    HashMap<&str, petgraph::graph::NodeIndex>,
    petgraph::graph::UnGraph<u32, u32>,
) {
    let mut nodes: HashMap<&str, petgraph::graph::NodeIndex> = HashMap::new();
    let mut graph = petgraph::graph::UnGraph::new_undirected();

    for line in i.lines() {
        let mut names = line.split("-");
        let n1 = names.next().unwrap();
        let n2 = names.next().unwrap();
        let idx1 = *nodes.entry(n1).or_insert_with(|| graph.add_node(1));
        let idx2 = *nodes.entry(n2).or_insert_with(|| graph.add_node(1));

        graph.add_edge(idx1, idx2, 1);
    }

    (nodes, graph)
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let (nodes, graph) = build_graph(input);

    let mut cnt = 0;
    for n in nodes.keys().filter(|n| n.starts_with("t")) {
        let e = graph.edges(nodes[n]).count();
        println!("{n}: {e}");
        for _e in graph.edges(nodes[n]) {
            //dbg!{e};
            cnt += 1;
        }
    }
    Ok(cnt)
}

fn run_2(_input: &str) -> anyhow::Result<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    #[ignore]
    fn day23_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 7);
    }

    #[test]
    fn day23_run_2() {}
}
