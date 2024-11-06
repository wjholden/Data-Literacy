use petgraph::dot::Dot;
use petgraph::graph::Graph;
use petgraph::algo::page_rank;
use petgraph::Undirected;

// https://depth-first.com/articles/2020/02/03/graphs-in-rust-an-introduction-to-petgraph/

fn main() {
    let mut g: Graph<&str, (), Undirected> = Graph::new_undirected();
    let frank = g.add_node("Frank");
    let kim = g.add_node("Kim");
    let david = g.add_node("David");
    let adin = g.add_node("Adin");
    let maria = g.add_node("Maria");
    let jose = g.add_node("Jose");
    let jasmine = g.add_node("Jasmine");
    let ivan = g.add_node("Ivan");
    let felix = g.add_node("Felix");
    let sven = g.add_node("Sven");
    let jean = g.add_node("Jean");
    let phil = g.add_node("Phil");

    g.extend_with_edges(&[
        (frank, kim),
        (frank, david),
        (david, sven),
        (david, adin),
        (adin, maria),
        (adin, jose),
        (adin, kim),
        (maria, jose),
        (jose, kim),
        (jasmine, jose),
        (jasmine, kim),
        (ivan, adin),
        (jean, ivan),
        (felix, ivan),
        (kim, felix),
        (kim, david),
        (kim, jean),
        (sven, adin),
        (phil, adin),
    ]);

    //let g = g;
    //println!("{}", Dot::new(&g));

    let mut pr = page_rank(&g, 0.75_f32, 10);
    pr.sort_by(|a,b| a.total_cmp(b));
    println!("{:?}", pr);
}
