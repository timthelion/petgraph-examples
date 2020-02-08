//! Example petgraph graphs
extern crate petgraph;
use petgraph::graph::Graph;

pub fn singleton() -> Graph<String, String, petgraph::Undirected> {
    let mut g = Graph::new_undirected();
    g.add_node("only value".to_string());
    g
}

pub fn list() -> Graph<String, String, petgraph::Undirected> {
    let mut g = Graph::new_undirected();
    let item1 = g.add_node("a".to_string());
    let item2 = g.add_node("b".to_string());
    let item3 = g.add_node("c".to_string());
    g.add_edge(item1, item2, "".to_string());
    g.add_edge(item2, item3, "".to_string());
    g
}

pub fn table() -> Graph<String, String, petgraph::Undirected> {
    let mut g = Graph::new_undirected();
    let cellA1 = g.add_node("A1".to_string());
    let cellA2 = g.add_node("A2".to_string());
    let cellA3 = g.add_node("A3".to_string());

    let cellB1 = g.add_node("B1".to_string());
    let cellB2 = g.add_node("B2".to_string());
    let cellB3 = g.add_node("B3".to_string());

    let cellC1 = g.add_node("C1".to_string());
    let cellC2 = g.add_node("C2".to_string());
    let cellC3 = g.add_node("C3".to_string());

    // Columns
    g.add_edge(cellA1, cellA2, "".to_string());
    g.add_edge(cellA2, cellA3, "".to_string());

    g.add_edge(cellB1, cellB2, "".to_string());
    g.add_edge(cellB2, cellB3, "".to_string());

    g.add_edge(cellC1, cellC2, "".to_string());
    g.add_edge(cellC2, cellC3, "".to_string());

    // Rows
    g.add_edge(cellA1, cellB1, "".to_string());
    g.add_edge(cellB1, cellC1, "".to_string());

    g.add_edge(cellA2, cellB2, "".to_string());
    g.add_edge(cellB2, cellC2, "".to_string());

    g.add_edge(cellA3, cellB3, "".to_string());
    g.add_edge(cellB3, cellC3, "".to_string());
    g
}

pub fn tree() -> Graph<String, String, petgraph::Directed> {
    let mut g = Graph::new();
    let tree_item1 = g.add_node("a".to_string());
    let tree_item2 = g.add_node("b".to_string());
    let tree_item3 = g.add_node("c".to_string());
    let tree_item4 = g.add_node("d".to_string());
    let tree_item5 = g.add_node("e".to_string());
    g.add_edge(tree_item1, tree_item2, "".to_string());
    g.add_edge(tree_item1, tree_item3, "".to_string());
    g.add_edge(tree_item2, tree_item4, "".to_string());
    g.add_edge(tree_item2, tree_item5, "".to_string());
    g
}

pub fn dag() -> Graph<String, String, petgraph::Directed> {
    let mut g = Graph::new();
    let dag_item1 = g.add_node("a".to_string());
    let dag_item2 = g.add_node("b".to_string());
    let dag_item3 = g.add_node("c".to_string());
    let dag_item4 = g.add_node("d".to_string());
    let dag_item5 = g.add_node("e".to_string());
    let dag_item6 = g.add_node("f".to_string());
    g.add_edge(dag_item1, dag_item2, "".to_string());
    g.add_edge(dag_item1, dag_item3, "".to_string());
    g.add_edge(dag_item2, dag_item4, "".to_string());
    g.add_edge(dag_item2, dag_item5, "".to_string());
    g.add_edge(dag_item4, dag_item6, "".to_string());
    g.add_edge(dag_item5, dag_item6, "".to_string());
    g
}

pub fn directed_graph_with_cycle() -> Graph<String, String, petgraph::Directed> {
    let mut g = Graph::new();
    let gwc_item1 = g.add_node("a".to_string());
    let gwc_item2 = g.add_node("b".to_string());
    let gwc_item3 = g.add_node("c".to_string());
    let gwc_item4 = g.add_node("d".to_string());
    let gwc_item5 = g.add_node("e".to_string());
    let gwc_item6 = g.add_node("f".to_string());
    g.add_edge(gwc_item1, gwc_item2, "".to_string());
    g.add_edge(gwc_item1, gwc_item3, "".to_string());
    g.add_edge(gwc_item2, gwc_item4, "".to_string());
    g.add_edge(gwc_item2, gwc_item5, "".to_string());
    g.add_edge(gwc_item4, gwc_item6, "".to_string());
    g.add_edge(gwc_item5, gwc_item6, "".to_string());
    g.add_edge(gwc_item6, gwc_item1, "".to_string());
    g
}

pub fn directed_graph_with_loop() -> Graph<String, String, petgraph::Directed> {
    let mut g = Graph::new();
    let gwc_item1 = g.add_node("a".to_string());
    let gwc_item2 = g.add_node("b".to_string());
    let gwc_item3 = g.add_node("c".to_string());
    let gwc_item4 = g.add_node("d".to_string());
    let gwc_item5 = g.add_node("e".to_string());
    let gwc_item6 = g.add_node("f".to_string());
    g.add_edge(gwc_item1, gwc_item2, "".to_string());
    g.add_edge(gwc_item1, gwc_item3, "".to_string());
    g.add_edge(gwc_item2, gwc_item4, "".to_string());
    g.add_edge(gwc_item2, gwc_item5, "".to_string());
    g.add_edge(gwc_item4, gwc_item6, "".to_string());
    g.add_edge(gwc_item5, gwc_item6, "".to_string());
    g.add_edge(gwc_item6, gwc_item6, "".to_string());
    g
}

pub fn ring() -> Graph<String, String, petgraph::Undirected> {
    let mut g = Graph::new_undirected();
    let ring_item1 = g.add_node("a".to_string());
    let ring_item2 = g.add_node("b".to_string());
    let ring_item3 = g.add_node("c".to_string());
    let ring_item4 = g.add_node("d".to_string());
    g.add_edge(ring_item1, ring_item2, "".to_string());
    g.add_edge(ring_item2, ring_item3, "".to_string());
    g.add_edge(ring_item3, ring_item4, "".to_string());
    g.add_edge(ring_item4, ring_item1, "".to_string());
    g
}

pub fn dict() -> Graph<String, String, petgraph::Directed> {
    let mut g = Graph::new();
    let core = g.add_node("dict".to_string());

    let key1 = g.add_node("key 1".to_string());
    let key2 = g.add_node("key 2".to_string());
    let key3 = g.add_node("key 3".to_string());

    let value1 = g.add_node("value 1".to_string());
    let value2 = g.add_node("value 2".to_string());
    let value3 = g.add_node("value 3".to_string());

    g.add_edge(core, key1, "".to_string());
    g.add_edge(core, key2, "".to_string());
    g.add_edge(core, key3, "".to_string());

    g.add_edge(key1, value1, "".to_string());
    g.add_edge(key2, value2, "".to_string());
    g.add_edge(key3, value3, "".to_string());
    g
}

use std::fmt;

#[derive(Debug, Clone)]
pub struct Person {
    name: String,
    age: u8,
}

#[derive(Debug, Copy, Clone)]
pub enum Relationship {
    Friend,
    Parent,
    Boss,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.name, self.age)
    }
}

impl fmt::Display for Relationship {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn social() -> Graph<Person, Relationship, petgraph::Directed> {
    let mut g = Graph::new();
    let bob = g.add_node(Person {
        name: "Bob".to_string(),
        age: 37,
    });
    let alice = g.add_node(Person {
        name: "Alice".to_string(),
        age: 17,
    });
    g.add_edge(bob, alice, Relationship::Parent);

    let lilly = g.add_node(Person {
        name: "Lilly".to_string(),
        age: 50,
    });
    g.add_edge(lilly, bob, Relationship::Boss);

    let george = g.add_node(Person {
        name: "George".to_string(),
        age: 16,
    });
    g.add_edge(george, alice, Relationship::Friend);
    g.add_edge(lilly, george, Relationship::Parent);

    let fred = g.add_node(Person {
        name: "Fred".to_string(),
        age: 16,
    });
    g.add_edge(george, fred, Relationship::Friend);
    g.add_edge(alice, fred, Relationship::Friend);
    g
}

pub fn multi_component() -> Graph<String, String, petgraph::Undirected> {
    let mut g = Graph::new_undirected();
    g.add_node("component a".to_string());
    g.add_node("component b".to_string());
    g
}
