extern crate gust;

use gust::traits::HasID;
use gust::Graph;

#[derive(Debug, HasID)]
struct Vertex {
  #[gust(id)]
  id: usize,
}

fn make_graph() -> Graph<Vertex> {
    let mut graph = Graph::new();
    graph.add_edge(21, 4);
    graph.add_edge(4, 21);
    graph.add_edge_bidi(100, 4);
    graph.add_edge(7, 43);
    graph
}

#[ignore]
#[test]
fn graph_len() {
    let graph = make_graph();
    println!("{:#?}", graph);
    assert_eq!(4, graph.length());
}

#[ignore]
#[test]
fn graph_vertices() {
  let graph = make_graph();
  let v = graph.vertices().next();
  println!("{:#?}", v);
}

#[test]
fn graph_get_adjacent() {
  let graph = make_graph();
  let adj = graph.get_adjacent(&4);
  assert_eq!(adj.len(), 2);
  assert!(adj.iter().any(|edge| edge.leads_from_to(&4, &21)));
  assert!(adj.iter().any(|edge| edge.leads_from_to(&4, &100)));
}

#[test]
fn graph_has_edge() {
  let graph = make_graph();
  assert!(graph.has_edge(&7, &43));
  assert!(graph.has_edge(&100, &4));
  assert!(!graph.has_edge(&7, &100));
}
