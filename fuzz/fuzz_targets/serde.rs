// This harness serializes and deserializes a graph
#![no_main]

use libfuzzer_sys::fuzz_target;
use util::{Edges, to_graph};

fuzz_target!(|data: Edges| {
    let graph = to_graph(data);
    let serialized = serde_json::to_string(&graph).unwrap();
    let _: petgraph::Graph<u32, u8> = serde_json::from_str(&serialized).unwrap();
});
