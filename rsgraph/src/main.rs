use graphrs::algorithms::centrality::closeness;
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

    let mut graph: Graph<&str, i32> = Graph::new(GraphSpecs::directed_create_missing());
    _ = graph.add_edges(vec![
        Edge::with_weight("start", "forest", 70.0),
        Edge::with_weight("start", "mountains", 60.0),
        Edge::with_weight("start", "sea", 54.0),
        Edge::with_weight("start", "city", 81.0),
        Edge::with_weight("forest", "start", 42.0),
        Edge::with_weight("forest", "mountains", 51.0),
        Edge::with_weight("forest", "desert", 56.0),
        Edge::with_weight("forest", "cave", 63.0),
        Edge::with_weight("mountains", "start", 71.0),
        Edge::with_weight("mountains", "forest", 38.0),
        Edge::with_weight("mountains", "glacier", 72.0),
        Edge::with_weight("desert", "forest", 93.0),
        Edge::with_weight("cave", "forest", 19.0),
        Edge::with_weight("cave", "inferno", 17.0),
        Edge::with_weight("inferno", "cave", 71.0),
        Edge::with_weight("glacier", "mountains", 25.0),
        Edge::with_weight("sea", "start", 49.0),
        Edge::with_weight("sea", "beach", 88.0),
        Edge::with_weight("beach", "sea", 79.0),
        Edge::with_weight("beach", "city", 29.0),
        Edge::with_weight("city", "beach", 30.0),
        Edge::with_weight("city", "start", 33.0),
        Edge::with_weight("city", "castle", 36.0),
        Edge::with_weight("castle", "city", 39.0),
        Edge::with_weight("castle", "treasure", 76.0),
        Edge::with_weight("treasure", "castle", 76.0),
    ]).unwrap();

    let betweenness_statistic = betweenness::betweenness_centrality(&graph, false, true).unwrap();
    let closeness_statistic = closeness::closeness_centrality(&graph, false, true).unwrap();
    let df: DataFrame = df!(
        "Name" => betweenness_statistic.clone().into_keys().collect::<Vec<_>>(),
        "Betweenness" => betweenness_statistic.clone().into_values().collect::<Vec<_>>(),
        "Closeness" => closeness_statistic.clone().into_values().collect::<Vec<_>>(),
    ).unwrap();
    println!("{}", df);

    //let closeness_statistic = closeness::closeness_centrality(&graph, false, true);
    
}
