// This harness constructs a graph randomly and finds a minimum spanning forest for it
#![no_main]

use libfuzzer_sys::fuzz_target;
use util::{Edges, to_graph};

fuzz_target!(|data: Edges| {
    let graph = to_graph(data);
    petgraph::algo::min_spanning_tree(&graph);
});
