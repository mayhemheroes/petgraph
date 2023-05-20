#![no_main]

use core::convert::From;
use arbitrary::{Arbitrary, Unstructured};
use libfuzzer_sys::fuzz_target;

use petgraph::algo::min_spanning_tree;
use petgraph::Graph;

/// This harness constructs a graph randomly and finds a minimum spanning forest for it

// To avoid running out of memory
const MAX_NODES: u32 = 1000u32;

#[derive(Debug)]
struct Edge(u32, u32, u8);

impl<'a> Arbitrary<'a> for Edge {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(Edge(
            u.int_in_range(0..=MAX_NODES - 1)?,
            u.int_in_range(0..=MAX_NODES - 1)?,
            u.arbitrary()?
        ))
    }
}

#[derive(Debug)]
struct Edges(Vec<(u32, u32, u8)>);

impl<'a> Arbitrary<'a> for Edges {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let len = u.arbitrary_len::<Edge>()?;
        let mut edges = Vec::new();
        for _ in 0..len {
            let Edge(from, to, weight) = u.arbitrary()?;
            edges.push((from, to, weight));
        }

        Ok(Edges(edges))
    }
}


fuzz_target!(|data: Edges| {
    let edges = data.0;
    let graph = Graph::<u32, u8>::from_edges(&edges);
    min_spanning_tree(&graph);
});
