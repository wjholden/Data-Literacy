use petgraph::dot::Dot;
use petgraph::graph::Graph;

fn main() {
    let mut g = Graph::new_undirected();
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
        (frank, kim, 1),
        (frank, david, 1),
        (david, sven, 1),
        (david, adin, 1),
        (adin, maria, 1),
        (adin, jose, 1),
        (adin, kim, 1),
        (maria, jose, 1),
        (jose, kim, 1),
        (jasmine, jose, 1),
        (jasmine, kim, 1),
        (ivan, adin, 1),
        (jean, ivan, 1),
        (felix, ivan, 1),
        (kim, felix, 1),
        (kim, david, 1),
        (kim, jean, 1),
        (sven, adin, 1),
        (phil, adin, 1),
    ]);

    println!("{}", Dot::new(&g));
}
