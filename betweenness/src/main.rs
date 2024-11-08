use std::collections::HashMap;

use petgraph::algo::astar;
use petgraph::prelude::*;
use num_rational::{self, Rational32};

fn main() {
    let mut graph: DiGraph<&str, f32> = Graph::new();
    let a = graph.add_node("Alice");
    let b = graph.add_node("Bob");
    let c = graph.add_node("Carol");
    let d = graph.add_node("Dan");
    let e = graph.add_node("Eve");
    let f = graph.add_node("Frank");
    let g = graph.add_node("Gale");
    graph.extend_with_edges(&[
        (a, c, 1.0), (b, c, 1.0), (c, d, 1.0),
        (c, e, 1.3), (d, f, 1.0), (e, f, 0.5),
        (f, g, 1.0),
    ]);
    let graph = graph;
    let result = betweenness(&graph);
    let lo = result.values().min().unwrap();
    let hi = result.values().max().unwrap();
    for v in result.keys() {
        println!("{:?}: {}", v, (result.get(v).unwrap() - lo) / (hi - lo));
    }
    // not quite. :-( 
}

fn betweenness(graph: &DiGraph<&str, f32>) -> HashMap<NodeIndex, Rational32> {
    let mut result = HashMap::new();
    for x in graph.node_indices() {
        let mut paths_with_x = 0;
        let mut paths_total = 0;
        for s in graph.node_indices() {
            if x == s {
                continue
            }
            for t in graph.node_indices() {
                if x == t || s == t {
                    continue
                }
                let result = astar(&graph, s, |finish| finish == t, |e| *e.weight(), |_| 0.0);
                match result {
                    Some((_, path)) => {
                        if path.contains(&x) {
                            paths_with_x += 1;
                        }
                        paths_total += 1;
                    },
                    None => ()
                };       
            }
        }
        result.insert(x, Rational32::new(paths_with_x, paths_total));
    }
    result
}
