use petgraph::algo::dijkstra;
use petgraph::prelude::*;
use petgraph::visit::NodeRef;
use petgraph::Graph;

fn main() {
    let mut graph: Graph<(), (), Undirected> = Graph::new_undirected();
    let a = graph.add_node(());
    let b = graph.add_node(());
    let c = graph.add_node(());
    let d = graph.add_node(());
    let e = graph.add_node(());
    graph.extend_with_edges(&[(a, b), (b, c), (c, d), (d, e)]);
    let graph = graph;
    closeness(&graph);
}

fn closeness<N, E>(graph: &Graph<N, E, Undirected>) {
    for u in graph.node_indices() {
        let delta = dijkstra(&graph, u.id(), None, |_| 1);
        let n = graph.node_count() as f64;
        let distances = delta.values().cloned().sum::<i32>() as f64;
        let closeness = (n - 1.0) / distances;
        println!(
            "Closeness score for vertex {} is {}.",
            u.index(),
            closeness
        );
    }
}
