use graphrs::{Edge, Graph, GraphSpecs};
use graphrs::algorithms::centrality::betweenness;
use polars::prelude::*;

fn main() {
    let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs::undirected_create_missing());
    _ = graph.add_edges(vec![
        Edge::new("Frank", "Kim"),
        Edge::new("Frank", "David"),
        Edge::new("David", "Sven"),
        Edge::new("David", "Adin"),
        Edge::new("Adin", "Maria"),
        Edge::new("Adin", "Jose"),
        Edge::new("Adin", "Kim"),
        Edge::new("Maria", "Jose"),
        Edge::new("Jose", "Kim"),
        Edge::new("Jasmine", "Jose"),
        Edge::new("Jasmine", "Kim"),
        Edge::new("Ivan", "Adin"),
        Edge::new("Jean", "Ivan"),
        Edge::new("Felix", "Ivan"),
        Edge::new("Kim", "Felix"),
        Edge::new("Kim", "David"),
        Edge::new("Kim", "Jean"),
        Edge::new("Sven", "Adin"),
        Edge::new("Phil", "Adin"),
    ]).unwrap();

    let betweenness_statistic = betweenness::betweenness_centrality(&graph, false, true).unwrap();
    let df: DataFrame = df!(
        "Name" => betweenness_statistic.clone().into_keys().collect::<Vec<_>>(),
        "Betweenness" => betweenness_statistic.clone().into_values().collect::<Vec<_>>(),
    ).unwrap();
    println!("{}", df);

    //let closeness_statistic = closeness::closeness_centrality(&graph, false, true);
    
}
