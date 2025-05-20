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

    let g = petgraph::dot::Dot::with_config(&graph, &[petgraph::dot::Config::EdgeNoLabel]);
    let _ = std::fs::write("g.dot", g.to_string().as_bytes());

    // let scc = petgraph::algo::tarjan_scc(&graph);
    // dbg!{&scc};

    let mut s = std::collections::HashSet::new();
    for n in nodes.keys().filter(|n| n.starts_with("t")) {
        let nbrs: Vec<_> = graph.neighbors(nodes[n]).collect();
        let mut trios = vec![nodes[n], nodes[n], nodes[n]];
        for nbr in nbrs.chunks(2) {
            trios[1] = nbr[0];
            trios[2] = nbr[1];
            trios.sort();
            s.insert(trios.clone());
            print!("{n} -> ");
            print!(
                "{} -> ",
                nodes
                    .iter()
                    .find_map(|(k, v)| if *v == nbr[0] { Some(k) } else { None })
                    .unwrap()
            );
            println!(
                "{}",
                nodes
                    .iter()
                    .find_map(|(k, v)| if *v == nbr[1] { Some(k) } else { None })
                    .unwrap()
            );
        }
    }
    Ok(s.len())
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
