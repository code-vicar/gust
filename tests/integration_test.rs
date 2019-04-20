extern crate gust;

use gust::traits::HasID;
use gust::{Graph, GraphBuilder};

#[derive(Debug, HasID)]
struct MyOwnVertex {
  #[gust(id)]
  id: usize,
  cool: String,
  data: String,
  bro: String
}

impl MyOwnVertex {
  fn new(id: usize) -> MyOwnVertex {
    MyOwnVertex {
      id,
      cool: String::from("cool"),
      data: String::from("data"),
      bro: String::from("bro")
    }
  }
}

#[derive(Debug)]
struct EdgeData {
    weight: u32
}

fn make_graph() -> Graph<MyOwnVertex, EdgeData> {
    let vertices = vec![
        MyOwnVertex::new(21),
        MyOwnVertex::new(4),
    ];

    let graph_builder = GraphBuilder::new()
        .with_vertices(vertices);

    let mut graph = graph_builder.build();
    graph.add_edge_with_data(&21, &4, Some(EdgeData {
        weight: 10
    }));
    graph.add_edge(&4, &21);
    graph
}

#[test]
fn graph_len() {
    let graph = make_graph();
    println!("{:#?}", graph);
    assert_eq!(2, graph.length());
}

#[test]
fn graph_vertices() {
  let graph = make_graph();
  let v = graph.vertices().iter().next();
  println!("{:#?}", v);
}
