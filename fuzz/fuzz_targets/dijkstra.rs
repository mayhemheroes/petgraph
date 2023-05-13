#![no_main]
use libfuzzer_sys::fuzz_target;
use petgraph::{prelude::UnGraph, algo::{dijkstra, bellman_ford, floyd_warshall, k_shortest_path::k_shortest_path}};

fuzz_target!(|edges: Vec<(u8, u8, f32)>| {
    if edges.len() > 1000 {
        return;
    }
    let edges: Vec<_> = edges.into_iter().filter(|(_, _, w)| !w.is_nan()).collect();
    let mut graph = UnGraph::<u8, f32>::new_undirected();
    let len = edges.len();
    for (from, to, weight) in edges {
        let a = graph.add_node(from);
        let b = graph.add_node(to);
        graph.add_edge(a, b, weight);
    }

    if len > 0 {
        dijkstra(&graph, graph.node_indices().next().unwrap(), Some(graph.node_indices().last().unwrap()), |_| 1);
        let _ = bellman_ford(&graph, graph.node_indices().next().unwrap());
        k_shortest_path(&graph, graph.node_indices().next().unwrap(), Some(graph.node_indices().last().unwrap()), 1, |_| 1);
    }
});