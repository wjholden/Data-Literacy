use petgraph::visit::NodeRef;
use petgraph::Graph;
use petgraph::prelude::*;
use petgraph::algo::dijkstra;

fn main() {
    let mut graph: Graph<(), (), Undirected> = Graph::new_undirected();
    let a = graph.add_node(());
    let b = graph.add_node(());
    let c = graph.add_node(());
    let d = graph.add_node(());
    let e = graph.add_node(());
    graph.extend_with_edges(&[
        (a,b),
        (b,c),
        (c,d),
        (d,e)
    ]);
    let graph = graph;
    closeness(&graph);
}

fn test2() {
    let mut graph: Graph<&str, (), Undirected> = Graph::new_undirected();
    let start = graph.add_node("start");
    let forest = graph.add_node("forest");
    let mountains = graph.add_node("mountains");
    let desert = graph.add_node("desert");
    let cave = graph.add_node("cave");
    let inferno = graph.add_node("inferno");
    let glacier = graph.add_node("glacier");
    let sea = graph.add_node("sea");
    let beach = graph.add_node("beach");
    let city = graph.add_node("city");
    let castle = graph.add_node("castle");
    let treasure = graph.add_node("treasure");
    graph.extend_with_edges(&[
        (start, forest),
        (start, mountains),
        (start, sea),
        (start, city),
        (forest, start),
        (forest, mountains),
        (forest, desert),
        (forest, cave),
        (mountains, start),
        (mountains, forest),
        (mountains, glacier),
        (desert, forest),
        (cave, forest),
        (cave, inferno),
        (inferno, cave),
        (glacier, mountains),
        (sea, start),
        (sea, beach),
        (beach, sea),
        (beach, city),
        (city, beach),
        (city, start),
        (city, castle),
        (castle, city),
        (castle, treasure),
        (treasure, castle)
    ]);
    closeness(&graph);
}

fn closeness<N, E>(graph: &Graph<N, E, Undirected>) {
    for u in graph.node_indices() {
        let delta = dijkstra(&graph, u.id(), None, |_| 1);
        let n = graph.node_count() as f64;
        let distances = delta.values().cloned().sum::<i32>() as f64;
        let closeness = (n-1.0) / distances;
        println!("Average distance for vertex {} is {}.", u.index(), closeness);
    }
}
