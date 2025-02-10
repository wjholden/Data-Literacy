use graphrs::algorithms::centrality::betweenness;
use graphrs::algorithms::centrality::closeness;
use graphrs::{Edge, Graph, GraphSpecs, Node};
use polars::prelude::*;

fn main() {
    betweenness_example();

    //let closeness_statistic = closeness::closeness_centrality(&graph, false, true);

    // https://neo4j.com/docs/graph-data-science/current/algorithms/betweenness-centrality/
    // https://snap.stanford.edu/class/cs224w-readings/brandes01centrality.pdf

    // https://neo4j.com/docs/graph-data-science/current/algorithms/closeness-centrality/
    // http://www.orgnet.com/MappingTerroristNetworks.pdf
}

/**

Try to recreate the 
https://neo4j.com/docs/graph-data-science/current/algorithms/betweenness-centrality/

## Create
```no_run
CREATE
  (alice:User {name: 'Alice'}),
  (bob:User {name: 'Bob'}),
  (carol:User {name: 'Carol'}),
  (dan:User {name: 'Dan'}),
  (eve:User {name: 'Eve'}),
  (frank:User {name: 'Frank'}),
  (gale:User {name: 'Gale'}),

  (alice)-[:FOLLOWS {weight: 1.0}]->(carol),
  (bob)-[:FOLLOWS {weight: 1.0}]->(carol),
  (carol)-[:FOLLOWS {weight: 1.0}]->(dan),
  (carol)-[:FOLLOWS {weight: 1.3}]->(eve),
  (dan)-[:FOLLOWS {weight: 1.0}]->(frank),
  (eve)-[:FOLLOWS {weight: 0.5}]->(frank),
  (frank)-[:FOLLOWS {weight: 1.0}]->(gale);
```

## Project
```no_run
MATCH (source:User)-[r:FOLLOWS]->(target:User)
RETURN gds.graph.project(
  'myGraph',
  source,
  target,
  { relationshipProperties: r { .weight } }
)
```

## Closeness centrality

Really close, but not exactly what I see in Rust...

```no_run
CALL gds.closeness.stream('myGraph')
YIELD nodeId, score
RETURN gds.util.asNode(nodeId).name as name, score
ORDER BY name ASC
```

## Betweenness centrality

This actually works! Same result in Neo4j and `graphrs`!

```no_run
CALL gds.betweenness.stream('myGraph', {
    samplingSize: 7,
    relationshipWeightProperty: 'weight'
})
YIELD nodeId, score
RETURN gds.util.asNode(nodeId).name AS name, score
ORDER BY name ASC
```

 */
fn betweenness_example() {
    let v = vec![
        Node::from_name("Alice"),
        Node::from_name("Bob"),
        Node::from_name("Carol"),
        Node::from_name("Eve"),
        Node::from_name("Dan"),
        Node::from_name("Frank"),
        Node::from_name("Gale"),
    ];

    let e = vec![
        Edge::with_weight("Alice", "Carol", 1.0),
        Edge::with_weight("Bob", "Carol", 1.0),
        Edge::with_weight("Carol", "Dan", 1.0),
        Edge::with_weight("Carol", "Eve", 1.3),
        Edge::with_weight("Dan", "Frank", 1.0),
        Edge::with_weight("Eve", "Frank", 0.5),
        Edge::with_weight("Frank", "Gale", 1.0),
    ];

    let g = Graph::<&str, ()>::new_from_nodes_and_edges(v, e, GraphSpecs::directed()).unwrap();
    let betweenness_statistic = betweenness::betweenness_centrality(&g, true, false).unwrap();
    let closeness_statistic = closeness::closeness_centrality(&g, false, false).unwrap();
    
    let df1: DataFrame = df!(
        "Name" => betweenness_statistic.clone().into_keys().collect::<Vec<&str>>(),
        "Betweenness" => betweenness_statistic.clone().into_values().collect::<Vec<f64>>(),
    ).unwrap();

    let df2: DataFrame = df!(
        "Name" => closeness_statistic.clone().into_keys().collect::<Vec<&str>>(),
        "Closeness" => closeness_statistic.clone().into_values().collect::<Vec<f64>>(),
    ).unwrap();

    //let mut df = df1.inner_join(&df2, [col("Name")], [col("Name")]).unwrap();

    for mut df in vec![df1, df2] {
        df.sort_in_place(["Name"], Default::default()).unwrap();
        println!("{}", df);
    }
}

 
#[allow(dead_code)]
fn kingdom() {
    let mut graph: Graph<&str, i32> = Graph::new(GraphSpecs::directed_create_missing());
    _ = graph
        .add_edges(vec![
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
        ])
        .unwrap();

    let betweenness_statistic = betweenness::betweenness_centrality(&graph, false, true).unwrap();
    let closeness_statistic = closeness::closeness_centrality(&graph, false, true).unwrap();
    let df: DataFrame = df!(
        "Name" => betweenness_statistic.clone().into_keys().collect::<Vec<_>>(),
        "Betweenness" => betweenness_statistic.clone().into_values().collect::<Vec<_>>(),
        "Closeness" => closeness_statistic.clone().into_values().collect::<Vec<_>>(),
    )
    .unwrap();
    println!("{}", df);
}

#[allow(dead_code)]
fn friends() {
    let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs::undirected_create_missing());
    _ = graph
        .add_edges(vec![
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
        ])
        .unwrap();
}
