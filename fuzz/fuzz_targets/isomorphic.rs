#![no_main]
use libfuzzer_sys::fuzz_target;
use petgraph::{prelude::UnGraph, algo::{is_isomorphic}};

fuzz_target!(|input: (Vec<(u8, u8, f32)>, Vec<(u8, u8, f32)>)| {
    let (edges_a, edges_b) = input;
    if edges_a.len() > 1000 || edges_b.len() > 1000 {
        return;
    }
    let edges_a: Vec<_> = edges_a.into_iter().filter(|(_, _, w)| !w.is_nan()).collect();
    let edges_b: Vec<_> = edges_b.into_iter().filter(|(_, _, w)| !w.is_nan()).collect();

    let mut graph_a = UnGraph::<u8, f32>::new_undirected();
    for (from, to, weight) in edges_a {
        let a = graph_a.add_node(from);
        let b = graph_a.add_node(to);
        graph_a.add_edge(a, b, weight);
    }

    let mut graph_b = UnGraph::<u8, f32>::new_undirected();
    for (from, to, weight) in edges_b {
        let a = graph_b.add_node(from);
        let b = graph_b.add_node(to);
        graph_b.add_edge(a, b, weight);
    }

    is_isomorphic(&graph_a, &graph_b);
});